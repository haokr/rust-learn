mod data;
mod simple_service;

use std::collections::HashMap;
use std::path::PathBuf;

pub use simple_service::SimpleService;
pub use data::*;

pub trait Service {
    /// 外部调用
    fn fetch(&self, req: Request) -> Response {
        match self.match_file(&req) {
            Some(files) => self.multi_match_file(&req, &files),
            None => Response { status: 200, message: "File Not Found".into(), ..Default::default() }
        }
    }
    
    /// 多文件匹配
    fn multi_match_file<'a>(&self, req: &Request, files: &Vec<PathBuf>) -> Response {
        let mut res:  HashMap<String, Vec<FetchResult>> = HashMap::new();
        for f in files {
            let match_res = self.match_str(req, f);
            match match_res {
                Ok(m) => match m {
                    Some(mr) => res.insert(f.file_name().unwrap().to_str().unwrap().to_string(), mr),
                    None => None
                },
                _ => None
            };
        }
        Response {
            status: 200,
            res,
            ..Default::default()
        }
    }

    /// 匹配正则表达式
    fn match_file(&self, req: &Request) -> Option<Vec<PathBuf>>;

    /// 匹配单个文件
    fn match_str(&self, req: &Request, file: &PathBuf) -> Result<Option<Vec<FetchResult>>, String>;
}