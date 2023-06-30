mod create;
mod delete;
mod edit;
mod get;

use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get))
            .route("edit", post().to(edit::edit)) // define view and url
            .route("delete", post().to(delete::delete)),
    );
}
