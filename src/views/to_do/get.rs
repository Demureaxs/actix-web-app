use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use actix_web::Responder;

pub async fn get(token: JwToken) -> impl Responder {
    return ToDoItems::get_state(token.user_id);
}
