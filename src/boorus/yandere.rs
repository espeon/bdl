use crate::BooruInfo;
use crate::ArrayResult;

pub async fn yandere(info: BooruInfo) -> anyhow::Result<String> {
    let image_result =
        reqwest::get(&format!("https://{}/post.json?tags=id:690092#{}", info.host, info.id))
            .await?
            .json::<ArrayResult>()
            .await.unwrap();
    dbg!(&image_result);
    Ok(image_result[0].file_url.clone())
}

pub async fn yandere_parser(url: url::Url, host:String) -> anyhow::Result<BooruInfo> {
    let pairs = url.path();
    let id = pairs.split("/").collect::<Vec<&str>>()[3].parse::<i64>().unwrap();
    let ret = BooruInfo {
        host: host,
        id: id,
    };

    Ok(ret)
}