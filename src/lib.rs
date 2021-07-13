pub mod client;

pub use client::Client;

pub mod common {
    tonic::include_proto!("common");
}
pub mod configservices {
    tonic::include_proto!("configservices");
}
pub mod core {
    tonic::include_proto!("core");
}
pub mod emane {
    tonic::include_proto!("emane");
}
pub mod mobility {
    tonic::include_proto!("mobility");
}
pub mod services {
    tonic::include_proto!("services");
}
pub mod wlan {
    tonic::include_proto!("wlan");
}
