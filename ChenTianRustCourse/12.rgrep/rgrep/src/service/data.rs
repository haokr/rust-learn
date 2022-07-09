use std::collections::HashMap;

use crate::cli;

#[derive(Debug)]
pub struct Request<'a> {
    pub cmd: &'a cli::Command,
}

#[derive(Debug, Default)]
pub struct Response {
    pub status: u16,
    pub message: String,
    // todo 用 trait 代替确定的类型
    pub res: HashMap<String, Vec<FetchResult>>
}

#[derive(Debug, Default)]
pub struct FetchResult {
    pub line: u32,
    pub pos: Vec<(usize, usize)>,
    pub content: String,
}
