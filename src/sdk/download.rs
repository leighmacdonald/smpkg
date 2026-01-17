use std::{fs::remove_file, os::unix::fs, path::Path};

use flate2::read::GzDecoder;
use reqwest::Error;
use tar::Archive;

pub async fn fetch_latest_version(branch: &String) -> Result<String, Error> {
    let target = format!("https://sm.alliedmods.net/smdrop/{branch}/sourcemod-latest-linux");
    reqwest::get(target).await?.text().await
}

pub async fn fetch_version(root: &Path, branch: String) -> Result<(), Box<dyn std::error::Error>> {
    print!("Fetching latest version... ");
    let version = fetch_latest_version(&branch).await?;
    println!("Found: {version}");
    let target = format!("https://sm.alliedmods.net/smdrop/{branch}/{version}");
    println!("Downlading sdk: {target}");
    let body = reqwest::get(target).await?.bytes().await?;
    let gz = GzDecoder::new(&body[..]);
    let mut archive = Archive::new(gz);
    let out_path = root.join(format!("sdks/sourcemod-{}", branch));
    println!("Extracting into: {:?}...", out_path);
    archive.unpack(out_path)?;

    activate_sdk(root, branch)?;

    Ok(())
}

pub fn get_installed_sdks(root: &Path) -> Vec<String> {
    let mut sdks = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root.join("sdks")) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("sourcemod-") {
                        sdks.push(name.to_string());
                    }
                }
            }
        }
    }
    sdks
}

pub fn activate_sdk(root: &Path, branch: String) -> Result<(), Box<dyn std::error::Error>> {
    let wanted = root.join(format!("sdks/sourcemod-{branch}"));
    let sdks = get_installed_sdks(root);
    if sdks.is_empty() {
        Err("No SDKs installed, try: sourcemod install".into())
    } else {
        let wanted_sdk = sdks.iter().find(|p| {
            println!(
                "eq: {} {}",
                wanted.display(),
                root.join("sdks").join(Path::new(p)).display()
            );
            wanted == root.join("sdks").join(Path::new(p))
        });
        println!("{wanted_sdk:?}");
        match wanted_sdk {
            Some(latest_sdk) => {
                let sm_root = root.join("sdks").join(Path::new(latest_sdk));
                let current_root = root.join("sdks/current");
                println!("Activating {latest_sdk} @ {current_root:?}");

                if current_root.exists() {
                    remove_file(current_root.clone())?;
                }

                let _ = fs::symlink(sm_root, current_root)?;
                Ok(())
            }
            None => Err("No SDK found for branch".into()),
        }
    }
}
