mod query_item;
mod query_user;
mod subscribe_item;
mod types;
mod query;
mod unsubscribe_item;

use napi::bindgen_prelude::*;
use query_user::WorkshopQueryUserTask;

use std::sync::Arc;
use steamworks::{AccountId, AppIDs, AppId, Client, UserList};
use subscribe_item::WorkshopSubscribeItemTask;
use types::*;
use unsubscribe_item::WorkshopUnsubscribeItemTask;

use query_item::WorkshopQueryItemTask;

// Workshop client for interacting with Steam Workshop
#[napi]
pub struct UGCClient {
  client: Arc<Client>,
}

#[napi]
impl UGCClient {
  pub fn new(client: Arc<Client>) -> Self {
    Self { client: client }
  }

  #[napi]
  pub fn item_state_to_string(&self, state: WorkshopItemStateEnum) -> &'static str {
    match state {
      WorkshopItemStateEnum::None => "None",
      WorkshopItemStateEnum::Subscribed => "Subscribed",
      WorkshopItemStateEnum::LegacyItem => "LegacyItem",
      WorkshopItemStateEnum::Installed => "Installed",
      WorkshopItemStateEnum::NeedsUpdate => "NeedsUpdate",
      WorkshopItemStateEnum::Downloading => "Downloading",
      WorkshopItemStateEnum::DownloadPending => "DownloadPending",
    }
  }

  #[napi]
  pub fn query_user(
    &self,
    app_id: u32, 
    account: u32, 
    list_type: WorkshopUserListType, 
    item_type: WorkshopUGCType, 
    sort_order: WorkshopUserListOrder, 
    page: u32
  ) -> AsyncTask<WorkshopQueryUserTask> {
    AsyncTask::new(WorkshopQueryUserTask {
      client:self.client.clone(),
      app_id: AppId(app_id),
      account: AccountId::from_raw(account),
      list_type: list_type.to_user_list(),
      item_type: item_type.to_ugc_type(),
      sort_order: sort_order.to_user_list_order(),
      page 
    })
  }

  #[napi]
  pub fn get_item(&self, item_id: f64) -> AsyncTask<WorkshopQueryItemTask> {
    AsyncTask::new(WorkshopQueryItemTask {
      client: self.client.clone(),
      item_id,
    })
  }

  #[napi]
  pub fn subscribe_to_item(&self, item_id: f64) -> AsyncTask<WorkshopSubscribeItemTask> {
    AsyncTask::new(WorkshopSubscribeItemTask {
      client: self.client.clone(),
      item_id,
    })
  }

  #[napi]
  pub fn unsubscribe_to_item(&self, item_id: f64) -> AsyncTask<WorkshopUnsubscribeItemTask> {
    AsyncTask::new(WorkshopUnsubscribeItemTask {
      client: self.client.clone(),
      item_id,
    })
  }

  #[napi]
  pub fn get_subscriptions(&self) -> Result<Vec<f64>> {
    let ugc = self.client.ugc();
    let subscribed_items = ugc.subscribed_items();
    let item_ids: Vec<f64> = subscribed_items.iter().map(|id| id.0 as f64).collect();

    Ok(item_ids)
  }

  #[napi]
  pub fn get_install_info(&self, item_id: f64) -> Result<Option<WorkshopItemInstallInfo>> {
    let ugc = self.client.ugc();
    let file_id = steamworks::PublishedFileId(item_id as u64);

    match ugc.item_install_info(file_id) {
      Some(install_info) => Ok(Some(WorkshopItemInstallInfo {
        folder: install_info.folder,
        size_on_disk: install_info.size_on_disk as f64,
        timestamp: install_info.timestamp,
      })),
      None => Ok(None),
    }
  }

  #[napi]
  pub fn get_download_info(&self, item_id: f64) -> Result<Option<DownloadInfo>> {
    let ugc = self.client.ugc();
    let file_id = steamworks::PublishedFileId(item_id as u64);

    if let Some((current, total)) = ugc.item_download_info(file_id) {
      Ok(Some(DownloadInfo {
        current: current as f64,
        total: total as f64,
      }))
    } else {
      Ok(None)
    }
  }

  #[napi]
  pub fn download_item(&self, item_id: f64, high_priority: bool) -> Result<bool> {
    let ugc = self.client.ugc();
    let file_id = steamworks::PublishedFileId(item_id as u64);
    Ok(ugc.download_item(file_id, high_priority))
  }

  #[napi]
  pub fn get_item_state(&self, item_id: f64) -> Result<u32> {
    let ugc = self.client.ugc();
    let file_id = steamworks::PublishedFileId(item_id as u64);
    let state = ugc.item_state(file_id);
    Ok(state.bits())
  }

  #[napi]
  pub fn get_item_states(&self, item_id: f64) -> Result<Vec<WorkshopItemStateEnum>> {
    let ugc = self.client.ugc();
    let file_id = steamworks::PublishedFileId(item_id as u64);

    let state = ugc.item_state(file_id);
    let mut states = Vec::new();

    // Check each possible state and add to the result vector if set
    if state.contains(steamworks::ItemState::SUBSCRIBED) {
      states.push(WorkshopItemStateEnum::Subscribed);
    }
    if state.contains(steamworks::ItemState::LEGACY_ITEM) {
      states.push(WorkshopItemStateEnum::LegacyItem);
    }
    if state.contains(steamworks::ItemState::INSTALLED) {
      states.push(WorkshopItemStateEnum::Installed);
    }
    if state.contains(steamworks::ItemState::NEEDS_UPDATE) {
      states.push(WorkshopItemStateEnum::NeedsUpdate);
    }
    if state.contains(steamworks::ItemState::DOWNLOADING) {
      states.push(WorkshopItemStateEnum::Downloading);
    }
    if state.contains(steamworks::ItemState::DOWNLOAD_PENDING) {
      states.push(WorkshopItemStateEnum::DownloadPending);
    }

    // If no states are found, add None
    if states.is_empty() {
      states.push(WorkshopItemStateEnum::None);
    }

    Ok(states)
  }
}
