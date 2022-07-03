use std::collections::HashMap;

use crate::cli;

#[derive(Debug)]
pub struct Request<'a> {
    pub cmd: &'a (cli::Command),
}

#[derive(Debug, Default)]
pub struct Response {
    pub status: u16,
    pub message: String,
    // todo 用 trait 代替确定的类型
    pub res: HashMap<String, Vec<Fetch_Result>>
}

#[derive(Debug, Default)]
pub struct Fetch_Result {
    pub line: u32,
    pub position: u32,
    pub content: String,
}
