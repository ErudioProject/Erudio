use std::process::Command;
fn main() {
	let output = Command::new("git")
		.args(["rev-parse", "HEAD"])
		.output()
		.expect("No git installed");
	let git_hash = String::from_utf8(output.stdout).expect("Strange git output");
	println!("cargo:rustc-env=GIT_HASH={}", git_hash);
	println!(
		"cargo:rustc-env=BUILD_DATE={}",
		chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
	);
}
