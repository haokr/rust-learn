use super::Service;


#[derive(Default)]
pub struct Simple_Service;


impl Service for Simple_Service {
    fn match_file(&self, req: &super::Request) -> Option<Vec<String>> {
        None
    }

    fn match_str(&self, req: &super::Request, file: &String) -> Option<Vec<super::Fetch_Result>> {
        None
    }
}