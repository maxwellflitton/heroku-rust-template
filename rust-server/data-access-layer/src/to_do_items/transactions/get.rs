use crate::to_do_items::schema::ToDoItem;
use nanoservices_utils::errors::NanoServiceError;
use std::future::Future;

#[cfg(feature = "json-file")]
use super::super::descriptors::JsonFileDescriptor;
#[cfg(feature = "json-file")]
use crate::json_file::get_all;
#[cfg(feature = "json-file")]
use std::collections::HashMap;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostGresDescriptor;
#[cfg(feature = "sqlx-postgres")]
use nanoservices_utils::errors::NanoServiceErrorStatus;


pub type GetAllResponse = Result<Vec<ToDoItem>, NanoServiceError>;

pub trait GetAll {
    fn get_all() -> impl Future<Output = GetAllResponse> + Send;
}


#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostGresDescriptor {
    fn get_all() -> impl Future<Output = GetAllResponse> + Send {
        sqlx_postgres_get_all()
    }
}


#[cfg(feature = "json-file")]
impl GetAll for JsonFileDescriptor {
    fn get_all() -> impl Future<Output = GetAllResponse> + Send {
        json_file_get_all()
    }
}


#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_all() -> GetAllResponse {
    let items = sqlx::query_as::<_, ToDoItem>("
        SELECT * FROM to_do_items
    ")
    .fetch_all(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    Ok(items)
}


#[cfg(feature = "json-file")]
async fn json_file_get_all() -> GetAllResponse {
    let tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let items: Vec<ToDoItem> = tasks.values().cloned().collect();
    Ok(items)
}
