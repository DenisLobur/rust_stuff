use super::db_access::*;
use super::models::Course;
use super::state::AppState;
use std::convert::TryFrom;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tuple = params.0;
    let tutor_id: i32 = i32::try_from(tuple).unwrap();
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let tuple = params;
    let tutor_id: i32 = i32::try_from(tuple.0).unwrap();
    let course_id: i32 = i32::try_from(tuple.1).unwrap();
    let course = get_course_details_db(
        &app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

