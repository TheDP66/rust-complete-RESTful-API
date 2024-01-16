use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::UserExt,
    dtos::{FilterUserDto, LoginUserDto, RegisterUserDto, UserData, UserResponseDto},
    error::{ErrorMessage, HttpError},
    utils::password,
    AppState,
};

pub fn auth_scope() -> Scope {
    web::scope("/api/auth").route("/register", web::post().to(register))
}

pub async fn register(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let hashed_password =
        password::hash(&body.password).map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state
        .db_client
        .save_user(&body.name, &body.email, &hashed_password)
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Created().json(UserResponseDto {
            status: "success".to_string(),
            data: UserData {
                user: FilterUserDto::filter_user(&user),
            },
        })),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_voilation(
                    ErrorMessage::EmailExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}
