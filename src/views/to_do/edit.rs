extern crate diesel;
use crate::database::establish_connection;
use crate::database::DB;
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use crate::schema::to_do;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    let mut connection = establish_connection();
    
    let results = to_do::table.filter(to_do::columns::title.eq(&to_do_item.title));
    
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&mut connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
