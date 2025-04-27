use napi::bindgen_prelude::*;
use napi::{Env, Result};
use std::sync::Arc;
use steamworks::{AccountId, AppIDs, AppId, QueryHandle, UGCType, UserList, UserListOrder};

use super::query::fetch_details;
use super::types::*;

// /// Async Task for getting workshop item details
pub struct QueryUserTask {
  pub client: Arc<steamworks::Client>,
  pub app_id: AppId,
  pub account: AccountId,
  pub list_type: UserList,
  pub item_type: UGCType,
  pub sort_order: UserListOrder,
  pub page: u32,
}

impl QueryUserTask {
  fn create_query_handle(&self) -> Result<QueryHandle> {
    match self.client.ugc().query_user(
      self.account,
      self.list_type,
      self.item_type,
      self.sort_order,
      AppIDs::Both {
        creator: self.app_id,
        consumer: self.app_id,
      },
      self.page,
    ) {
      Ok(handle) => Ok(handle),
      Err(e) => Err(Error::from_reason(e.to_string())),
    }
  }
}

#[napi]
impl Task for QueryUserTask {
  type Output = Option<Vec<WorkshopItemDetails>>;
  type JsValue = Option<Vec<WorkshopItemDetails>>;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let query_handle = self.create_query_handle()?;
    let details = fetch_details(query_handle)?;
    Ok(Some(details))
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }

  fn reject(&mut self, _: Env, error: Error) -> Result<Self::JsValue> {
    Err(error)
  }
}
