use tonic::{transport::Server, Request, Response, Status};

use signuis::authentication_server::{Authentication, AuthenticationServer};

pub mod signuis {
    tonic::include_proto!("signuis"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

}