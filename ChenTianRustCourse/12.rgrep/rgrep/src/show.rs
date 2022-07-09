use std::collections::HashMap;
use crate::service::FetchResult;
use colored::Colorize;

pub fn show(data: &HashMap<String, Vec<FetchResult>>) {
    data.iter().for_each(|e| {
        if e.1.is_empty() {
            return;
        }
        println!("{}", format!("{}", e.0.blue()));
        for l in e.1 {
            let mut line = "\t".to_owned();
            let mut end: usize = 0;
            for p in &l.pos {
                line.push_str( &l.content[end..p.0]);
                line.push_str(format!("{}", l.content[p.0..p.1].red().bold()).as_str());
                end = p.1;
            }
            if end < l.content.len() {
                line.push_str(&l.content[end..l.content.len()]);
            }
            let ps = l.pos.iter().map(|e| e.0.to_string()).collect::<Vec<String>>();
            let pss = ps.join(".");
            println!("    {}:{} {}", l.line, pss ,line);
        }
    })
}