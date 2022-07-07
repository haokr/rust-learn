mod data;
mod simple_service;

use std::collections::HashMap;

pub use simple_service::Simple_Service;
pub use data::*;

pub trait Service {
    fn fetch(&self, req: Request) -> Response {
        match self.match_file(&req) {
            Some(files) => self.multi_match_file(&req, &files),
            None => Response { status: 200, ..Default::default() }
        }
    }
    
    fn multi_match_file<'a>(&self, req: &Request, files: &Vec<String>) -> Response {
        let mut res:  HashMap<String, Vec<Fetch_Result>> = HashMap::new();
        for f in files {
            let match_res = self.match_str(&req, f);
            match match_res {
                Some(mr) => res.insert(f.clone(), mr),
                None => None
            };
        }
        Response {
            status: 200,
            res,
            ..Default::default()
        }
    }

    fn match_file(&self, req: &Request) -> Option<Vec<String>>;

    fn match_str(&self, req: &Request, file: &String) -> Option<Vec<Fetch_Result>>;
}