use crate::connection;
use crate::posts;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts",
               routes![posts::handler::all]
                    // posts::handler::get,
                    // posts::handler::post,
                    // posts::handler::put,
                    // posts::handler::delete],
        ).launch();
}
