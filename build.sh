zip_dir() {
	local dir=$1
	pushd $dir >/dev/null
		zip --symlinks \
			"kill-process.alfredworkflow" \
			$(find .) \
			$(exclude_args)
	popd >/dev/null
	mv ./alfred-workflow/kill-process.alfredworkflow ./
}

cargo build --release --target x86_64-apple-darwin && cp -f ./target/x86_64-apple-darwin/release/kill_process ./alfred-workflow && zip_dir ./alfred-workflow
