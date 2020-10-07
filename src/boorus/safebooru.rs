use crate::BooruInfo;

use serde;
use serde::{Deserialize, Serialize};

pub type SafebooruArrayResult = Vec<SafebooruResult>;
#[derive(Debug, Serialize, Deserialize)]
pub struct SafebooruResult {
    id:i64,
    directory:String,
    image: String,
}

pub async fn safebooru(info: BooruInfo) -> anyhow::Result<String> {
    let image_result = reqwest::get(&format!("https://safebooru.org/index.php?page=dapi&s=post&q=index&id={}&json=1", info.id))
            .await?
            .json::<SafebooruArrayResult>()
            .await?;
    let res = format!("https://safebooru.org/images/{}/{}", image_result[0].directory, image_result[0].image);
    Ok(res)
}