use std::fs::File;
use reqwest::blocking::{Client, multipart::Form};
use crate::models::*;
use crate::simple_error::SimpleError;
use anyhow::{Result, bail, Error};
use reqwest::StatusCode;
use std::path::PathBuf;

pub struct FiledropApiClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl FiledropApiClient<'_> {
    pub fn new() -> Self {
        FiledropApiClient { client: Client::new(), base_url: "https://api.beta.filedrop.is" }
    }

    pub fn create_drop(&self) -> Result<CreateDropResponse> {
        let res = self.client.post(format!("{}/drops", self.base_url))
            .send()
            .unwrap();

        if res.status() != StatusCode::OK && res.status() != StatusCode::CREATED {
            bail!(Error::msg("Failed to get dropId"));
        }

        let drop = res.json::<CreateDropResponse>().unwrap();

        Ok(drop)
    }

    pub fn upload_file(&self, drop_id: &str, file_path: &PathBuf, access_token: &str) -> Result<()> {
        let form = Form::new()
            .text("fileSize", "0")
            .file("file", file_path)
            .unwrap();

        let res = self.client.post(format!("{}/drops/{}/files", self.base_url, drop_id))
            .multipart(form)
            .header("Temporary-Access-Token", access_token)
            .send()
            .unwrap();

        if res.status() != StatusCode::CREATED && res.status() != StatusCode::OK {
            bail!("Failed to upload file with status {}", res.status());
        }

        Ok(())
    }
}
