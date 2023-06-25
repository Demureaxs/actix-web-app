extern crate diesel;
use crate::database::establish_connection;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use diesel::prelude::*;

pub async fn create(req: HttpRequest) -> HttpResponse {
    // let state: Map<String, Value> = read_file("./state.json"); // step 1
    // let title: String = req.match_info().get("title").unwrap().to_string(); // step 2
    // let item = to_do_factory(&title.as_str(), TaskStatus::PENDING); // step 3

    // process_input(item, "create".to_string(), &state); // step 4

    // return HttpResponse::Ok().json(ToDoItems::get_state());

    // -------------------DB rather than Local state ------------------
    let title: String = req.match_info().get("title").unwrap().to_string();
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection);
    }

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
