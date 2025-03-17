use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::{PgPool, PgQueryResult};

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    // Prepare SQL statement
    let tutor_rows = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM eazy_schema.ezy_tutor_c6"
    )
    .fetch_all(pool)
    .await?;
    // Extract result
    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name.clone(),
            tutor_pic_url: tutor_row.tutor_pic_url.clone(),
            tutor_profile: tutor_row.tutor_profile.clone(),
        })
        .collect();
    match tutors.len() {
        0 => Err(EzyTutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM eazy_schema.ezy_tutor_c6 where tutor_id = $1",
        tutor_id
        )
        .fetch_one(pool)
        .await
        .map(|tutor_row|
            Tutor {
                tutor_id: tutor_row.tutor_id,
                tutor_name: tutor_row.tutor_name,
                tutor_pic_url: tutor_row.tutor_pic_url,
                tutor_profile: tutor_row.tutor_profile,
            }
        )
        .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;
    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "insert into eazy_schema.ezy_tutor_c6 (
            tutor_name,
            tutor_pic_url,
            tutor_profile
            ) values ($1,$2,$3) returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name,
        new_tutor.tutor_pic_url,
        new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
}

pub async fn update_tutor_details_db(
    db_pool: &PgPool,
    tutor: UpdateTutor,
    tutor_id: i32,
) -> Result<Vec<Tutor>, EzyTutorError> {
    let mut existing_tutor = sqlx::query_as!(
        Tutor,
        r#"SELECT tutor_id, tutor_name as "tutor_name", tutor_pic_url as "tutor_pic_url", tutor_profile as "tutor_profile" 
        FROM eazy_schema.ezy_tutor_c6 
        WHERE tutor_id = $1"#,
        tutor_id as i32)
        .fetch_one(db_pool)
        .await
        .map_err(|_error| EzyTutorError::NotFound(format!("Tutor id `{tutor_id}` not found.")))?;

    // Change the existing tutor fields if there is data
    if tutor.tutor_name.is_some() {
        existing_tutor.tutor_name = tutor.tutor_name.unwrap();
    }

    if tutor.tutor_pic_url.is_some() {
        existing_tutor.tutor_pic_url = tutor.tutor_pic_url.unwrap();
    }

    if tutor.tutor_profile.is_some() {
        existing_tutor.tutor_profile = tutor.tutor_profile.unwrap();
    }

    // Prepare the SQL update statement
    let updated_tutor_result = sqlx::query_as!(
        Tutor,
        r#"UPDATE eazy_schema.ezy_tutor_c6 SET 
        tutor_name = $1,
        tutor_pic_url = $2,
        tutor_profile = $3 
        WHERE tutor_id= $4 
        RETURNING 
        tutor_id,
        tutor_name as "tutor_name",
        tutor_pic_url as "tutor_pic_url",
        tutor_profile as "tutor_profile" "#,
        existing_tutor.tutor_name,
        existing_tutor.tutor_pic_url,
        existing_tutor.tutor_profile,
        tutor_id as i32
    )
    .fetch_one(db_pool)
    .await;

    match updated_tutor_result {
        Ok(up_tutor) => Ok(vec![up_tutor]),
        Err(_error) => Err(EzyTutorError::NotFound(format!(
            "Tutor id `{tutor_id}` not found"
        ))),
    }
} // end fn update_tutor_details_db()

pub async fn delete_tutor_db(db_pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    // Prepare the SQL delete statement
    let query_result: PgQueryResult = sqlx::query!(
        r#"DELETE FROM eazy_schema.ezy_tutor_c6
        WHERE tutor_id = $1 "#,
        tutor_id as i32
    )
    .execute(db_pool)
    .await?;

    match query_result.rows_affected() {
        0 => Err(EzyTutorError::NotFound(format!(
            "Tutor id `{tutor_id}` not found"
        ))),
        _ => Ok(format!("Deleted tutor `{tutor_id}`")),
    }
} // end fn delete_tutor_db()
