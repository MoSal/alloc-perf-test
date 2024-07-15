use speedy::{Readable, Writable};
use crate::storage_util::StorageInfo;

#[derive(Readable, Writable)]
pub struct Category {
    category_id: u64,
    category_name: String,
}

#[derive(Readable, Writable)]
pub struct BooiesCategories(Vec<Category>);

impl StorageInfo for BooiesCategories {
    const DESC: &'static str = "a list of all booies stweem categories";
}
