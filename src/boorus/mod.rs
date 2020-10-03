pub mod gelbooru;
pub mod danbooru;
pub mod yandere;

use serde;
use serde::{Deserialize, Serialize};

pub type ArrayResult = Vec<Result>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    id: i64,
    file_url: String,
}