use data_access_layer::to_do_items::schema::ToDoItem;
use nanoservices_utils::errors::NanoServiceError;
use data_access_layer::to_do_items::transactions::update::UpdateOne;


pub async fn update<T: UpdateOne>(item: ToDoItem, user_id: i32) -> Result<(), NanoServiceError> {
    let _ = T::update_one(item, user_id).await?;
    Ok(())
}
