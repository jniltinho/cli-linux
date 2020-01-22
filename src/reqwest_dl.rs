// Copyright (C) 2018, 2019, 2020 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

pub fn run_download(url: &str) {
    match dl_url_http(url.to_string()) {
        Err(error) => println!("{:?}", error),
        Ok(ok) => println!("{}", ok),
    }
}

#[tokio::main]
async fn dl_url_http(url: String) -> Result<String, anyhow::Error> {
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ));
        }
    };

    let mut request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));

    let file = Path::new(Path::new(url.as_str()).file_name().unwrap());

    if file.exists() {
        let size = file.metadata()?.len().saturating_sub(1);
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut source = request.send().await?;
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .await?;
    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    return Ok(format!(
        "Download of '{}' has been completed.",
        file.to_str().unwrap()
    ));

    //Ok(())
}
