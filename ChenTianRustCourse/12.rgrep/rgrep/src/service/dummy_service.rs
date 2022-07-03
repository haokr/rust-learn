use super::Service;


#[derive(Default)]
pub struct Dummy_Service;

impl Service for Dummy_Service {
    fn fetch(req: super::Request) -> super::Response {
        super::Response {
            status: 510,
            message: "Dummy Service".into(),
            ..Default::default()
        }
    }
}