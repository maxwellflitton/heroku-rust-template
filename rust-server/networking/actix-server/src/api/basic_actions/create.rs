use core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use nanoservices_utils::errors::NanoServiceError;
use data_access_layer::to_do_items::schema::{NewToDoItem, ToDoItem};
use data_access_layer::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use actix_web::{
    HttpResponse,
    web::Json
};


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn create<T>(body: Json<NewToDoItem>) 
    -> Result<HttpResponse, NanoServiceError>
where
    T: SaveOne + GetAll,
{
    let _ = create_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>().await?))
}
