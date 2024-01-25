use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::UserExt,
    dtos::{FilterUserDto, RequestQueryDto, UserData, UserListResponseDto, UserResponseDto},
    error::HttpError,
    extractors::auth::{Authenticated, RequireAuth},
    models::UserRole,
    AppState,
};

async fn get_me(user: Authenticated) -> Result<HttpResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user);

    let resposne_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: filtered_user,
        },
    };

    Ok(HttpResponse::Ok().json(resposne_data))
}

async fn get_users(
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(UserListResponseDto {
        status: "success".to_string(),
        users: FilterUserDto::filter_users(&users),
        results: users.len(),
    }))
}

pub fn users_scope() -> Scope {
    web::scope("/api/users")
        .route(
            "/me",
            web::get().to(get_me).wrap(RequireAuth::allowed_roles(vec![
                UserRole::User,
                UserRole::Moderator,
                UserRole::Admin,
            ])),
        )
        .route(
            "",
            web::get()
                .to(get_users)
                .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin])),
        )
}
