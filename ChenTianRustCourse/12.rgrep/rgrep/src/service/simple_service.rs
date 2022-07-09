use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

use super::{FetchResult, Service};
use regex::{Regex};
use crate::service::Request;

#[derive(Default)]
pub struct SimpleService;


impl Service for SimpleService {
    fn match_file(&self, req: &super::Request) -> Option<Vec<PathBuf>> {
        let files_path = Self::get_files();
        let res = Self::filter_file(req, files_path);
        Some(res)
    }

    fn match_str(&self, req: &super::Request, file: &PathBuf) -> Result<Option<Vec<FetchResult>>, String> {
        let mut res: Vec<FetchResult> = vec!();
        if !file.is_file() {
            return Ok(None);
        }
        let file_obj = File::open(file).unwrap();
        let lines = BufReader::new(file_obj).lines();
        Self::match_lines(req, &mut res, lines);
        if res.is_empty() {
            return Ok(None);
        }
        Ok(Some(res))
    }
}

impl SimpleService {
    fn get_files() -> Vec<PathBuf> {
        let current_dir = std::env::current_dir();
        let mut files_path: Vec<PathBuf> = Vec::new();
        match current_dir {
            Ok(c) => {
                let p = c.as_path();
                for f in std::fs::read_dir(p).unwrap() {
                    files_path.push(f.unwrap().path());
                }
            }
            _ => ()
        };
        files_path
    }

    fn filter_file(req: &Request, files_path: Vec<PathBuf>) -> Vec<PathBuf> {
        let reg = Regex::new(req.cmd.file.as_str()).unwrap();
        let res = files_path.into_iter().filter(|p| {
            reg.is_match(p.to_str().unwrap())
        }).collect::<Vec<PathBuf>>();
        res
    }

    fn match_lines(req: &Request, res: &mut Vec<FetchResult>, lines: Lines<BufReader<File>>) {
        let mut line_num = 0;
        for line in lines {
            line_num = line_num + 1;
            let content = line.unwrap();
            let reg = Regex::new(req.cmd.targe_string.as_str()).unwrap();

            let find_res = reg.find_iter(content.as_str());
            let mut pos: Vec<(usize, usize)> = vec!();
            for fr in find_res {
                pos.push((fr.start(), fr.end()))
            }
            if pos.is_empty() {
                continue;
            }
            res.push(FetchResult {
                line: line_num,
                pos,
                content: content.to_owned(),
            });
        }
    }
}