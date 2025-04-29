use napi::bindgen_prelude::*;
use napi::{Env, Result};
use std::sync::Arc;
use steamworks::{PublishedFileId, QueryHandle};

use super::query::fetch_details;
use super::types::WorkshopItemDetails;

/// Async Task for getting workshop item details
pub struct WorkshopQueryItemTask {
  pub client: Arc<steamworks::Client>,
  pub item_id: f64,
}

impl WorkshopQueryItemTask {
  fn create_query_handle(&self) -> Result<QueryHandle> {
    match self.client.ugc().query_item(PublishedFileId(self.item_id as u64)) {
      Ok(handle) => Ok(handle),
      Err(e) => Err(Error::from_reason(e.to_string())),
    }
  }
}

#[napi]
impl Task for WorkshopQueryItemTask {
  type Output = Option<WorkshopItemDetails>;
  type JsValue = Option<WorkshopItemDetails>;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let query_handle = self.create_query_handle()?;
    let details = fetch_details(query_handle)?;
    let first = details.into_iter().nth(0).unwrap();
    Ok(Some(first))
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }

  fn reject(&mut self, _: Env, error: Error) -> Result<Self::JsValue> {
    Err(error)
  }
}
