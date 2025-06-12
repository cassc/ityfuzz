use std::process::Command;
fn main() {
    let git_hash = Command::new("git")
        .args(["describe", "--always", "--dirty"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".into());

    let features = {
        let mut features = String::new();

        for (k, v) in std::env::vars() {
            if k.starts_with("CARGO_FEATURE_") && v == "1" {
                features.push_str(&format!("{},", &k[14..]));
            }
        }
        features.to_ascii_lowercase()
    };

    println!("cargo:rustc-env=GIT_VERSION_INFO={}[{:?}]", git_hash.trim(), features);
}
