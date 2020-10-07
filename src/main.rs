mod boorus;

use anyhow::anyhow;
use dotenv::dotenv;
use std::env;
use std::path::Path;
use structopt::StructOpt;
use url::{Url};

use tokio::fs;

use crate::boorus::*;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Download { url: String },
    Dl { url: String },
}

pub struct BooruInfo {
    host: String,
    id: i64,
}

#[paw::main]
#[tokio::main]
async fn main(args: Args) -> anyhow::Result<()> {
    dotenv().ok();
    match args.cmd {
        Some(Command::Download { url }) => {
            scrape(url).await?;
        }
        Some(Command::Dl { url }) => {
            scrape(url).await?;
        }
        None => {
            println!("i don't think this is what you're supposed to do...\nrun the 'help' command for information on how to use this program");
        }
    }

    Ok(())
}

async fn scrape(urlin: String) -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let url = Url::parse(&urlin)?;

    //parse and display info we get from the URL
    let info = parse_booru_info(url.clone()).await?;
    println!("checking image id {} at host {}", info.id, info.host);

    let mut base_path = env::var("SAVE_PATH").unwrap_or("".to_string());
    let divider: &str = &env::var("DIVIDER").unwrap_or("-".to_string());

    //check if destination folder exists, and if not create it
    match Path::new(&base_path.to_owned()).exists() {
        true => (),
        false => fs::create_dir(&base_path).await?,
    }

    //check if we should include the host as a folder
    match env::var("INCLUDE_HOST")
        .unwrap_or("true".to_string())
        .parse::<bool>()
        .unwrap()
    {
        true => {
            base_path = base_path + &url.host_str().unwrap() + divider;
        }
        false => print!(""),
    };

    //check if destination folder exists, and if not create it
    match Path::new(&base_path.to_owned()).exists() {
        true => (),
        false => fs::create_dir(&base_path).await?,
    }

    //add the post id to the file path
    let save_loc = base_path + &info.id.to_string();

    //check whether our image exists and if so exit
    //if not, attempt to get the image url and download it
    let img_exists = Path::new(&(save_loc.to_owned() + &".jpg")).exists()
        || Path::new(&(save_loc.to_owned() + &".png")).exists();
    println!(
        "checking '{}' for current images (jpg = {} | png = {} | both = {})",
        Path::new(&(save_loc.to_owned() + &".jpg")).display(),
        Path::new(&(save_loc.to_owned() + &".jpg")).exists(),
        Path::new(&(save_loc.to_owned() + &".png")).exists(),
        img_exists
    );
    match img_exists {
        true => {
            println!("An image is currently downloaded there.");
            return Err(anyhow!("An image is currently downloaded there."));
        }
        false => download(info, save_loc).await?,
    };
    Ok(())
}

async fn parse_booru_info(url: Url) -> anyhow::Result<BooruInfo> {
    let host = url.host_str().unwrap().to_string();
    let host_str:&str = &host;
    let ret = match host_str {
        "gelbooru.com" => gelbooru::gelbooru_parser(url, host).await?,
        "safebooru.org" => gelbooru::gelbooru_parser(url, host).await?, //uses gelbooru's id parser
        "danbooru.donmai.us" => danbooru::danbooru_parser(url, host).await?,
        "yande.re" => yandere::yandere_parser(url, host).await?,
        _ => return Err(anyhow!("Input url is not currently supported.")),
    };

    Ok(ret)
}

async fn download(info: BooruInfo, location: String) -> anyhow::Result<()> {
    let host: &str = &info.host; //borrowing here so it can be used in match statement
    let download_url = match host {
        "gelbooru.com" => gelbooru::gelbooru(info).await?,
        "safebooru.org" => safebooru::safebooru(info).await?,
        "danbooru.donmai.us" => danbooru::danbooru(info).await?,
        "yande.re" => yandere::yandere(info).await?,
        _ => return Err(anyhow!("Input url is not currently supported.")),
    };

    //not sure what this does, i copied it off the web
    let response = reqwest::get(&download_url).await?;
    let dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.png");

        let mut file_name_split = fname.split(".").collect::<Vec<&str>>(); // temp value dropped while borrowed if i unwrapped here

        let file_type = file_name_split.last_mut().unwrap();

        let fname = location + ".";
        let fname = fname + file_type;
        println!("file downloaded, will be located at '{:#?}'", fname);
        fname
    };
    let bytes = response.bytes().await?;
    let mut out = fs::File::create(dest).await?;
    tokio::io::copy(&mut &*bytes, &mut out).await?;
    Ok(())
}
