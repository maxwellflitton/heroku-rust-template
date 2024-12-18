use data_access_layer::to_do_items::schema::AllToDOItems;
use data_access_layer::to_do_items::transactions::get::GetAll;
use nanoservices_utils::errors::NanoServiceError;


pub async fn get_all<T: GetAll>() -> Result<AllToDOItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllToDOItems::from_vec(all_items)
}
