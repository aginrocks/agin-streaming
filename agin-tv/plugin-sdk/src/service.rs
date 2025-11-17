use crate::plugin::{SearchRequest, Service};

pub trait AginService {
    fn metadata(&self) -> Service;
}

pub trait SearchService {
    fn search(request: SearchRequest);
}
