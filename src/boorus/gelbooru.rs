use crate::BooruInfo;
use crate::ArrayResult;

pub async fn gelbooru(info: BooruInfo) -> anyhow::Result<String> {
    // get image result and parse it into an ImageResult struct
    let image_result =
        reqwest::get(&format!("https://{}/index.php?page=dapi&s=post&q=index&id={}&json=1", info.host, info.id))
            .await?
            .json::<ArrayResult>()
            .await?;
    // send the file url back
    Ok(image_result[0].file_url.clone())
}

pub async fn gelbooru_parser(url: url::Url, hosti:String) -> anyhow::Result<BooruInfo> {
    let mut pairs = url.query_pairs(); // parse the url
    let mut id = 0; // init the id variable
    while id == 0 { // take things off stack until we find the id
        let result = pairs.next();
        let unwrap = result.unwrap();
        if unwrap.clone().0.to_mut() == "id" {
            id = unwrap.clone().1.to_mut().parse::<i64>().unwrap()
        }
    }
    let ret = BooruInfo { // put host and id in a struct
        host: hosti,
        id: id,
    };

    Ok(ret) // return the struct
}