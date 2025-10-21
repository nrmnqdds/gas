pub mod pb {
    tonic::include_proto!("grpc.gomaluum_auth.unaryecho");
}

use pb::{EchoRequest, EchoResponse};
use tonic::{Request, Response, Status, metadata::MetadataValue};

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct EchoServer {}

#[tonic::async_trait]
impl pb::echo_server::Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }
}

pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let secret_token = std::env::var("GOMALUUM_AUTH_TOKEN");

    if secret_token.is_err() {
        return Err(Status::internal(
            "Server misconfiguration: missing auth token",
        ));
    }

    let token: MetadataValue<_> = format!("Bearer {}", secret_token.unwrap()).parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
