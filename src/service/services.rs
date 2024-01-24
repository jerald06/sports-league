use crate::entity::models::GroundDetails;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::MySqlPool;

#[post("/register")]
async fn add_ground_details(
    ground: web::Json<GroundDetails>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let ground_id = &ground.ground_id;
    let ground_name = &ground.ground_name;
    let ground_address = &ground.ground_address;

    let result = sqlx::query(
        r#"
        INSERT INTO grounds_details (ground_id,ground_name,ground_address) VALUES (?, ?, ?)
    "#,
    )
    .bind(ground_id)
    .bind(ground_name)
    .bind(ground_address)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("created successfully"),
        Err(err) => {
            eprintln!("Failed to insert grounds_details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to insert grounds_details: {:?}", err))
        }
    }
}

#[get("/ground/{ground_name}")]
async fn get_ground_details(path: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let ground_name = path.into_inner(); // Extract the String from web::Path

    // Prepare and execute the SQL query to fetch ground details by ID
    let result = sqlx::query_as::<_, GroundDetails>(
        r#"
        SELECT ground_id, ground_name, ground_address FROM grounds_details WHERE ground_name = ?
    "#,
    )
    .bind(&ground_name)
    .fetch_one(pool.get_ref())
    .await;

    // Handle the result and return the appropriate response
    match result {
        Ok(ground) => HttpResponse::Ok().json(ground),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Ground not found"),
        Err(err) => {
            eprintln!("Failed to fetch ground details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to fetch ground details: {:?}", err))
        }
    }
}

#[put("/update_ground/{id}")]
async fn update_ground_details(
    path: web::Path<String>,
    updated_ground: web::Json<GroundDetails>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let id = path.into_inner(); // Extract the String from web::Path

    let updated_ground_id = &updated_ground.ground_id;
    let updated_ground_name = &updated_ground.ground_name;
    let updated_ground_address = &updated_ground.ground_address;

    // Prepare and execute the SQL query to update ground details by ID
    let result = sqlx::query(
        r#"
        UPDATE grounds_details SET ground_id = ?, ground_name = ?, ground_address = ? WHERE ground_id = ?
    "#,
    )
        .bind(updated_ground_id)
        .bind(updated_ground_name)
        .bind(updated_ground_address)
        .bind(&id)
        .execute(pool.get_ref())
        .await;

    // Handle the result and return the appropriate response
    match result {
        Ok(_) => HttpResponse::Ok().body("Ground details updated successfully"),
        Err(err) => {
            eprintln!("Failed to update ground details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to update ground details: {:?}", err))
        }
    }
}
#[get("/get_all_grounds")]
async fn get_all_grounds(pool: web::Data<MySqlPool>) -> HttpResponse {
    // Prepare and execute the SQL query to fetch all ground details
    let result = sqlx::query_as::<_, GroundDetails>(
        r#"
        SELECT ground_id, ground_name, ground_address FROM grounds_details
    "#,
    )
    .fetch_all(pool.get_ref())
    .await;

    // Handle the result and return the appropriate response
    match result {
        Ok(grounds) => HttpResponse::Ok().json(grounds),
        Err(err) => {
            eprintln!("Failed to fetch all ground details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to fetch all ground details: {:?}", err))
        }
    }
}

#[delete("/delete_ground/{ground_name}")]
async fn delete_ground_by_name(
    path: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let ground_name = path.into_inner(); // Extract the String from web::Path

    // Prepare and execute the SQL query to delete ground details by ground_name
    let result = sqlx::query(
        r#"
        DELETE FROM grounds_details WHERE ground_name = ?
    "#,
    )
    .bind(&ground_name)
    .execute(pool.get_ref())
    .await;

    // Handle the result and return the appropriate response
    match result {
        Ok(_) => HttpResponse::Ok().body("Ground details deleted successfully"),
        Err(err) => {
            eprintln!("Failed to delete ground details: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Failed to delete ground details: {:?}", err))
        }
    }
}
