#[actix_web::patch("/projects/{user_login}/{project_id}/active")]
pub async fn set_active_route(
    req: actix_web::HttpRequest,
    json: actix_web::web::Json<RequestBody>,
    path: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
    user_sdk: actix_web::web::Data<authin_sdk::user::UserSdk>,
) -> impl actix_web::Responder {
    use actix_web::HttpResponse;
    use authin_sdk::user::{Token, authorize::AuthorizeParams, info::InfoParams};

    let client = client.into_inner();
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("")
    };

    let params = AuthorizeParams {
        token: Token(token.clone()),
        permission: config.service_permission.clone()
    };
    let _ = match user_sdk.authorize(params).await {
        Ok(false) | Err(_) => return HttpResponse::Unauthorized().body(""),
        _ => ()
    };
    
    let params = InfoParams {
        token: Token(token),
    };
    let _ = match user_sdk.get_info(params).await {
        Ok(user) => {
            if user.login != path.user_login {
                return HttpResponse::Unauthorized().body("");
            } 
        }
        Err(_) => return HttpResponse::Unauthorized().body(""),
    };

    match todoin_application::ProjectRepository::set_active(&path.project_id, &json.value, &*client).await {
        Ok(_) => return HttpResponse::Ok().body(""),
        Err(_) => return HttpResponse::BadRequest().body(""),
    };
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RequestBody {
    value: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RequestPath {
    user_login: String,
    project_id: i32,
}
