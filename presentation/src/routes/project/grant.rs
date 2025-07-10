#[actix_web::patch("/projects/grant")]
pub async fn grant_route(
    req: actix_web::HttpRequest,
    json: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
    user_sdk: actix_web::web::Data<authin_sdk::user::UserSdk>,
) -> impl actix_web::Responder {
    use actix_web::HttpResponse;
    use authin_sdk::user::{Token, authorize::AuthorizeParams};

    let client = client.into_inner();
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("")
    };

    let params = AuthorizeParams {
        token: Token(token),
        permission: config.service_permission.clone()
    };
    let _ = match user_sdk.authorize(params).await {
        Ok(false) | Err(_) => return HttpResponse::Unauthorized().body(""),
        _ => ()
    };
    
    match todoin_application::ProjectRepository::grant(&json.project_id, &json.user_login, &*client).await {
        Ok(_) => return HttpResponse::Ok().body(""),
        Err(_) => return HttpResponse::BadRequest().body(""),
    };
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone, Debug)]
pub struct RequestBody {
    project_id: i32,
    user_login: String
}
