use diesel::result::Error;
use std::env;
use rocket_contrib::json::Json;
use crate::posts;
use crate::posts::Post;
use crate::connection::DbConn;
use rocket::http::Status;
use rocket::response::status;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Post>>, Status> {
    posts::repository::all(&connection)
        .map(|posts| Json(posts))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Post>, Status> {
    posts::repository::get(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<post>")]
pub fn post(post: Json<Post>, connection: DbConn) -> Result<status::Created<Json<Post>>, Status> {
    posts::repository::insert(post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

fn post_created(post: Post) -> status::Created<Json<Post>> {
    status::Created(
        format!("{host}:{port}/posts/{id}", host = host(), port = port(), id = post.id).to_string(),
        Some(Json(post)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<post>")]
pub fn put(id: i32, post: Json<Post>, connection: DbConn) -> Result<Json<Post>, Status> {
    posts::repository::update(id, post.into_inner(), &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match posts::repository::get(id, &connection) {
        Ok(_) => posts::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
