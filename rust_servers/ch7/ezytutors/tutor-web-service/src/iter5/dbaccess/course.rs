use crate::errors::EzyTutorError;
use crate::models::course::*;
use sqlx::postgres::PgPool;
use sqlx::query_as;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare SQL statement
    let course_rows: Vec<Course> = query_as!(
        Course,
        "SELECT * FROM eazy_schema.ezy_course_c6 where tutor_id = $1",
        tutor_id
    )
        .fetch_all(pool)
        .await?;

    Ok(course_rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    // Prepare SQL statement
    let course_row = query_as!(
        Course,
        "SELECT * FROM eazy_schema.ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;
    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, EzyTutorError> {
    let course_row = query_as!(
        Course,
        "insert into eazy_schema.ezy_course_c6 (tutor_id, course_name, course_description,course_duration,
        course_level, course_format, course_language, course_structure,
        course_price) values ($1,$2,$3,$4,$5,$6,$7,$8,$9) returning
        tutor_id, course_id,course_name, course_description,
        course_duration, course_level, course_format, course_language,
        course_structure, course_price, posted_time",
        new_course.tutor_id,
        new_course.course_name,
        new_course.course_description,
        new_course.course_duration,
        new_course.course_level,
        new_course.course_format,
        new_course.course_language,
        new_course.course_structure,
        new_course.course_price
    )
        .fetch_one(pool)
        .await?;

    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    // Prepare SQL statement
    let course_row = sqlx::query!(
        "DELETE FROM eazy_schema.ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
        .execute(pool)
        .await?;

    Ok(format!("Deleted {:#?} record", course_row))
}

pub async fn update_course_details_db(db_pool: &PgPool, tutor_id: i32, course_id: i32, course: UpdateCourse) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare the SQL select statement to find the existing course

    let mut existing_course = query_as!(
        Course,
        r#"SELECT course_id, tutor_id, course_name as course_name, posted_time, course_description as "course_description?", course_format as "course_format?", course_structure as "course_structure?", course_duration as "course_duration?", course_language as "course_language?", course_level as "course_level?", course_price as "course_price?"
        FROM eazy_schema.ezy_course_c6
        WHERE tutor_id = $1 AND course_id = $2"#,
        tutor_id as i32,
        course_id as i32
    )
        .fetch_one(db_pool)
        .await
        .map_err(|_error| EzyTutorError::NotFound(format!("Course id `{course_id}` not found.")))?;

    // Change the existing course fields if there is data
    if course.course_name.is_some() {
        existing_course.course_name = course.course_name.unwrap();
    }

    if course.course_description.is_some() {
        existing_course.course_description = course.course_description;
    }

    if course.course_format.is_some() {
        existing_course.course_format = course.course_format;
    }

    if course.course_structure.is_some() {
        existing_course.course_structure = course.course_structure;
    }

    if course.course_duration.is_some() {
        existing_course.course_duration = course.course_duration;
    }

    if course.course_language.is_some() {
        existing_course.course_language = course.course_language;
    }

    if course.course_level.is_some() {
        existing_course.course_level = course.course_level;
    }

    if course.course_price.is_some() {
        existing_course.course_price = course.course_price;
    }

    // Prepare the SQL update statement
    let updated_course_result = query_as!(
        Course,
        r#"UPDATE eazy_schema.ezy_course_c6 SET
        course_name = $1, course_description = $2, course_format = $3,
        course_structure = $4, course_duration = $5, course_language = $6,
        course_level = $7, course_price = $8
        WHERE tutor_id = $9 AND course_id = $10
        RETURNING
        course_id, tutor_id, course_name as course_name, posted_time, course_description as "course_description?", course_format as "course_format?", course_structure as "course_structure?", course_duration as "course_duration?", course_language as "course_language?", course_level as "course_level?", course_price as "course_price?" "#,
        existing_course.course_name, existing_course.course_description, existing_course.course_format,
        existing_course.course_structure, existing_course.course_duration, existing_course.course_language,
        existing_course.course_level, existing_course.course_price,
        existing_course.tutor_id, existing_course.course_id
    )
        .fetch_one(db_pool)
        .await;

    match updated_course_result {
        Ok(up_course) => Ok(vec![up_course]),
        Err(_error) => Err(EzyTutorError::NotFound(format!("Course id `{course_id}` not found")))
    }
}
