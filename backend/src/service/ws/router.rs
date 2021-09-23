use crate::service::ws::{WSClient, WSServer, WsBizHandlers};
use actix::Addr;

use crate::service::user::LoggedUser;
use actix_web::{
    get,
    web::{Data, Path, Payload},
    Error,
    HttpRequest,
    HttpResponse,
};
use actix_web_actors::ws;

#[get("/{token}")]
pub async fn establish_ws_connection(
    request: HttpRequest,
    payload: Payload,
    token: Path<String>,
    server: Data<Addr<WSServer>>,
    biz_handlers: Data<WsBizHandlers>,
) -> Result<HttpResponse, Error> {
    match LoggedUser::from_token(token.clone()) {
        Ok(user) => {
            let client = WSClient::new(&user.user_id, server.get_ref().clone(), biz_handlers);
            let result = ws::start(client, &request, payload);
            match result {
                Ok(response) => Ok(response.into()),
                Err(e) => {
                    log::error!("ws connection error: {:?}", e);
                    Err(e)
                },
            }
        },
        Err(e) => {
            if e.is_unauthorized() {
                Ok(HttpResponse::Unauthorized().json(e))
            } else {
                Ok(HttpResponse::BadRequest().json(e))
            }
        },
    }
}
