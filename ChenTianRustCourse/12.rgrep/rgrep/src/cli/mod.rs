mod command;
use clap::Parser;
use crate::service::{Service, Response, Request};

pub use self::command::*;

pub struct Cli<S>
where S: Service
{
    cmd: Command,
    service: S,
}

impl<S> Cli<S> 
where S: Service
{
    pub fn new(s: S) -> Self {
        Self {
            cmd: Command::parse(),
            service: s
        }
    }

    pub fn run(&self) -> Response {
        let req = Request {
            cmd: &self.cmd
        };
        <S as Service>::fetch(req)
    }
}
