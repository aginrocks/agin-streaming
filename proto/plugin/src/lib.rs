pub mod impls;

mod proto {
    tonic::include_proto!("plugin");
}

pub use proto::*;
