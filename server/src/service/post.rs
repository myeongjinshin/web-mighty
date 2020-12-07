use crate::db;
use crate::db::user::{LoginForm, RegisterForm};
use actix_identity::Identity;
use actix_web::{http, post, web, Error, HttpResponse, Responder};
use deadpool_postgres::Pool;

#[post("/login")]
pub async fn login(id: Identity, form: web::Form<LoginForm>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let user_no = db::user::login(&form, &db_pool).await?;
    id.remember(user_no.to_string());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}

#[post("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
}

#[post("/register")]
pub async fn register(
    id: Identity,
    form: web::Form<RegisterForm>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let _ = db::user::register(&form, &db_pool).await?;
    id.remember(form.user_id.clone());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}
