/*
    This file is a part of alloc-perf-test.

    Copyright (C) 2024 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal

    alloc-perf-test is free software: you can redistribute it and/or modify
    it under the terms of the Affero GNU General Public License as
    published by the Free Software Foundation.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    Affero GNU General Public License for more details.

    You should have received a copy of the Affero GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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
