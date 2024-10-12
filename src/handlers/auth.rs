use actix_web::{web, HttpResponse, Responder, Error, HttpRequest};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::user::{CreateUser, LoginUser, User};
use crate::jwt::{create_token, verify_token, get_token_from_headers};
use chrono::Utc;

pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Form<CreateUser>,
) -> Result<HttpResponse, Error> {
    let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();
    let now = Utc::now();

    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        form.username,
        form.email,
        hashed_password,
        now
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => {
            let token = create_token(user.id)?;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "user": user,
                "token": token
            })))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to create user")),
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    form: web::Form<LoginUser>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        form.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => {
            if verify(&form.password, &user.password_hash).unwrap() {
                // Update last_login
                let now = Utc::now();
                let _ = sqlx::query!(
                    "UPDATE users SET last_login = $1 WHERE id = $2",
                    now,
                    user.id
                )
                .execute(pool.get_ref())
                .await;

                let token = create_token(user.id)?;
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "message": "Login successful",
                    "token": token
                })))
            } else {
                Ok(HttpResponse::Unauthorized().json("Invalid credentials"))
            }
        },
        Ok(None) => Ok(HttpResponse::Unauthorized().json("User not found")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Login failed")),
    }
}

pub async fn account(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let token = get_token_from_headers(&req)?;
    let claims = verify_token(&token)?;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json("User not found")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to fetch user data")),
    }
}