use crate::api::FiledropApiClient;
use anyhow::{bail, Error, Result};
use std::fs::metadata;
use std::path::PathBuf;

pub fn upload(files: &Vec<PathBuf>) -> Result<()> {
    let client = FiledropApiClient::new();

    let drop = client.create_drop()?;
    let access_token = &drop
        .access_token
        .ok_or(Error::msg("Failed to get access token"))?;
    for file in files {
        let name = file
            .file_name()
            .ok_or(Error::msg("Failed to get name of file"))?
            .to_str()
            .ok_or(Error::msg("Invalid filename"))?;

        println!("Uploading file {}", &name);
        client.upload_file(&drop.id, file, access_token)?;
    }

    println!("New drop created at https://beta.filedrop.is/{}", &drop.id);

    Ok(())
}
