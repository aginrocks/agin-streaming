use crate::plugin::Service;

pub trait AginService {
    fn metadata(&self) -> Service;
}
