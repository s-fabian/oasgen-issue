use actix_web::web::{Json, Query};
use actix_web::{App, HttpServer};
use oasgen::{oasgen, OaSchema, Server};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, OaSchema)]
pub struct GetCode {
    pub code: String,
}

#[derive(Serialize, OaSchema)]
pub struct CodeResponse {
    pub found_code: bool,
}

#[oasgen]
async fn get_code(Query(GetCode { code }): Query<GetCode>) -> Json<CodeResponse> {
    Json(CodeResponse {
        found_code: matches!(&*code, "1234" | "5678"),
    })
}

#[tokio::main]
async fn main() {
    let server = Server::actix().get("/get-code", get_code).freeze();
    HttpServer::new(move || App::new().service(server.clone().into_service()))
        .bind("0.0.0.0:5000")
        .unwrap()
        .run()
        .await
        .unwrap()
}
