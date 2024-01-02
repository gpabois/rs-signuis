use tonic::Status;


pub fn into_status(error: signuis_core:: Error) -> Status {
    Status::new(tonic::Code::Internal, error.message())
}