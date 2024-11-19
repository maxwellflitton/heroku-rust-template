use data_access_layer::to_do_items::schema::{NewToDoItem, ToDoItem};
use data_access_layer::to_do_items::transactions::create::SaveOne;
use nanoservices_utils::errors::NanoServiceError;


pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let created_item = T::save_one(item).await?;
    Ok(created_item)
}
