use flate2::read::GzDecoder;
use reqwest::Error;
use tar::Archive;

pub async fn fetch_latest_version(branch: &String) -> Result<String, Error> {
    let target = format!("https://sm.alliedmods.net/smdrop/{branch}/sourcemod-latest-linux");
    reqwest::get(target).await?.text().await
}

pub async fn fetch_version(branch: String) -> Result<(), Box<dyn std::error::Error>> {
    let version = fetch_latest_version(&branch).await?;
    let target = format!("https://sm.alliedmods.net/smdrop/{branch}/{version}");
    let body = reqwest::get(target).await?.bytes().await?;
    let gz = GzDecoder::new(&body[..]);
    let mut archive = Archive::new(gz);
    archive.unpack(format!(".sdk/sourcemod-{}", branch))?;
    Ok(())
}
