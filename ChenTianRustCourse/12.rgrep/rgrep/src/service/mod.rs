mod data;
pub mod dummy_service;

pub use data::*;

pub trait Service {
    fn fetch(req: Request) -> Response;
}