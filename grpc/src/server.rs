use signuis_core::log::info;
use tonic::transport::Server;

pub mod services;
pub mod codegen;
pub mod middlewares;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    signuis_core::config::Config::init_with_args(Default::default()).unwrap();

    let addr = "[::1]:5051".parse()?;
    let pool = signuis_core::services::ServicePool::from_config().await.unwrap();

    let reflection = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(codegen::FILE_DESCRIPTOR_SET)
    .build()
    .unwrap();

    info!("Starting gRPC server, listenning to {addr}");
    Server::builder()
        .layer(middlewares::AuthenticationLayer(pool.clone()))
        .add_service(reflection)
        .add_service(codegen::authentication_server::AuthenticationServer::new(services::Authentication(pool.clone())))
        .serve(addr)
        .await?;

    Ok(())
}