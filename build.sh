zip_dir() {
	local dir=$1
	pushd $dir >/dev/null
		zip --symlinks \
			"slay.alfredworkflow" \
			$(find .) \
			$(exclude_args)
	popd >/dev/null
	mv ./alfred-workflow/slay.alfredworkflow ./
}

cargo build --release --target x86_64-apple-darwin && cp -f ./target/x86_64-apple-darwin/release/slay ./alfred-workflow && zip_dir ./alfred-workflow
