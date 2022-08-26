use plist::Value;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{collections::HashMap, env, process::Command};

#[derive(Serialize, Deserialize)]
enum ApplicationType {
    Executable,
    Service,
    Application,
}

impl Default for ApplicationType {
    fn default() -> Self {
        ApplicationType::Executable
    }
}

#[derive(Serialize, Deserialize)]
struct CopyText {
    copy: String,
}

#[derive(Serialize, Deserialize)]
struct Icon {
    path: String,
}

#[derive(Serialize, Deserialize)]
struct ParsedApplication<'proclist> {
    title: String,
    subtitle: String,
    uid: &'proclist str,
    text: CopyText,
    arg: &'proclist str,
    icon: Icon,

    #[serde(skip)]
    app_type: ApplicationType,
}

#[derive(Serialize)]
struct AlfredList<'proclist> {
    items: Vec<ParsedApplication<'proclist>>,
}

struct IconParser<'proclist> {
    cached_icons: HashMap<&'proclist str, String>,
}

impl<'proclist> IconParser<'proclist> {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "default",
            "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/ExecutableBinaryIcon.icns"
                .to_string(),
        );
        Self { cached_icons: map }
    }

    fn get_app_icon_path(&mut self, app_path: &'proclist str) -> String {
        if app_path.is_empty() {
            return self.cached_icons.get("default").unwrap().to_string();
        }
        if let Some(icon_path) = self.cached_icons.get(app_path) {
            return icon_path.to_string();
        }

        let manifest_filepath = format!("{}/Contents/Info.plist", app_path);
        if !Path::new(&manifest_filepath).exists() {
            return self.cached_icons.get("default").unwrap().to_string();
        }

        let manifest = Value::from_file(&manifest_filepath)
            .unwrap_or_else(|_| panic!("failed to read {}", &manifest_filepath));

        if let Some(icon_name) = manifest
            .as_dictionary()
            .and_then(|dict| dict.get("CFBundleIconFile"))
            .and_then(|name| name.as_string())
        {
            // have to do this because some manifests don't include the file extension
            let icon_path = format!(
                "{}/Contents/Resources/{}.icns",
                app_path,
                icon_name.replace(".icns", "")
            );
            self.cached_icons.insert(app_path, icon_path.to_string());
            return icon_path;
        };
        self.cached_icons.get("default").unwrap().to_string()
    }
}

impl<'proclist> ParsedApplication<'proclist> {
    fn new(
        pid: &'proclist str,
        cpu: &'proclist str,
        path: &'proclist str,
        icon_parser: &mut IconParser<'proclist>,
    ) -> Self {
        let split_path: Vec<&str> = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();

        let path_start = split_path[0].to_string();
        // if path contains `.app` > 1 time, it's a helper process
        // E.G.: /Applications/Visual Studio Code.app/Contents/Frameworks/Code Helper.app/Contents/MacOS/Code Helper
        let apps: Vec<&str> = split_path
            .iter()
            .copied()
            .filter(|segment| segment.contains(".app"))
            .collect();
        let is_helper = apps.len() > 1;

        let full_application_path = &path.to_string();
        let app_path = match path.find(".app") {
            Some(index) => &path[0..index + 4],
            None => "",
        };
        let text = CopyText {
            copy: format!(
                "pid: {}, cpu {}%, path: {}",
                pid, cpu, &full_application_path
            ),
        };

        if path_start == *"Applications" || path_start == *"System" {
            let mut result = Self {
                uid: pid,
                arg: pid,
                text,
                title: (if apps.is_empty() {
                    full_application_path
                } else {
                    apps.last().copied().unwrap()
                })
                .to_string(),
                subtitle: format!(
                    "{}% CPU @ {}",
                    cpu,
                    if apps.len() > 1 { path } else { app_path }
                ),
                icon: Icon {
                    path: icon_parser.get_app_icon_path(app_path),
                },
                app_type: ApplicationType::Executable,
            };
            if is_helper || split_path[1] == "Library" {
                result.app_type = ApplicationType::Service;
                return result;
            };
            result.app_type = ApplicationType::Application;
            return result;
        }

        let title: Vec<&str> = path.trim().split(' ').collect();

        Self {
            uid: pid,
            arg: pid,
            title: (&title[0]).to_string(),
            subtitle: format!("{}% CPU @ {}", cpu, full_application_path),
            text: CopyText {
                copy: format!(
                    "pid: {}, cpu {}%, path: {}",
                    pid, cpu, full_application_path
                ),
            },
            icon: Icon {
                path: icon_parser.get_app_icon_path(app_path),
            },
            app_type: ApplicationType::Executable,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut results = AlfredList { items: Vec::new() };
    if args.len() < 2 {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return;
    }
    let query = &args[1].to_lowercase();

    let ps_output = Command::new("ps")
        .args(vec!["-A", "-o", "pid", "-o", "%cpu", "-o", "args"])
        .output()
        .unwrap();

    let process_list = String::from_utf8_lossy(&ps_output.stdout).to_string();
    let re = Regex::new(r"(?i)(\d+)\s+(\d+[\.|,]\d+)\s+(.*)").unwrap();
    let matching_lines: Vec<&str> = process_list
        .split('\n')
        .filter(|line| {
            if let Some(caps) = re.captures(line) {
                let path: Vec<&str> = caps
                    .get(3)
                    .map_or("", |m| m.as_str())
                    .trim()
                    .split(" -")
                    .collect();
                if path.is_empty() {
                    return false;
                }
                let path_str = path[0].to_lowercase();
                if path_str.contains(&format!("-{}", query)) || path_str.contains("kill_process") {
                    return false;
                }
                return path_str.contains(query);
            };
            false
        })
        .collect();
    let mut icon_parser = IconParser::new();
    for line in &matching_lines {
        let caps = re.captures(line).unwrap();
        let pid = caps.get(1).map_or("", |m| m.as_str());
        let cpu = caps.get(2).map_or("", |m| m.as_str());
        let path = caps.get(3).map_or("", |m| m.as_str()).trim();
        results
            .items
            .push(ParsedApplication::new(pid, cpu, path, &mut icon_parser));
    }
    results.items.sort_by_key(|item| match item.app_type {
        ApplicationType::Application => -1,
        ApplicationType::Service => 0,
        ApplicationType::Executable => 1,
    });

    println!("{}", serde_json::to_string_pretty(&results).unwrap());
}
