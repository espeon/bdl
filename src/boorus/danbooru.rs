use crate::BooruInfo;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DanbooruResult {
    id: i64,
    file_url: String,
}

pub async fn danbooru(info: BooruInfo) -> anyhow::Result<String> {
    let image_result =
        reqwest::get(&format!("https://{}/posts/{}.json", info.host, info.id))
            .await?
            .json::<DanbooruResult>()
            .await?;
    Ok(image_result.file_url)
}

pub async fn danbooru_parser(url: url::Url) -> anyhow::Result<BooruInfo> {
    let pairs = url.path();
    let id = pairs.split("/").collect::<Vec<&str>>()[2].parse::<i64>().unwrap();
    let ret = BooruInfo {
        host: url.host_str().unwrap().to_string(),
        id: id,
    };

    Ok(ret)
}