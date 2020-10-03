use crate::BooruInfo;
use crate::Result;

pub async fn danbooru(info: BooruInfo) -> anyhow::Result<String> {
    let image_result = reqwest::get(&format!("https://{}/posts/{}.json", info.host, info.id))
        .await?
        .json::<Result>()
        .await?;
    Ok(image_result.file_url)
}

pub async fn danbooru_parser(url: url::Url, host: String) -> anyhow::Result<BooruInfo> {
    let pairs = url.path();
    let id = pairs.split("/").collect::<Vec<&str>>()[2]
        .parse::<i64>()
        .unwrap();
    let ret = BooruInfo { host: host, id: id };

    Ok(ret)
}
