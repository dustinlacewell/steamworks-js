#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod ugc;
mod friends;
mod errors;

use napi::bindgen_prelude::*;
use steamworks::{Client, SteamId};
use ugc::*;
use friends::*;
use std::sync::Arc;

// Basic user info
#[napi(object)]
pub struct UserInfo {
  pub steam_id: f64,
  pub account_id: u32,
  pub name: String,
  pub state: PersonaStateEnum,
  pub level: u32,
  pub logged_on: bool,
}

#[napi]
pub struct SteamClient {
  client: Arc<Client>,
  ugc: Arc<UGCClient>,
  friends: FriendsClient,
}

#[napi]
impl SteamClient {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    match Client::init() {
      Ok(_client) => {
        let client = Arc::new(_client);
        let ugc = Arc::new(UGCClient::new(client.clone()));
        let friends = FriendsClient::new(client.clone());
        let steam_client = SteamClient { client: client.clone(), ugc, friends };
        SteamClient::start_pumping_callbacks_internal(client.clone());
        Ok(steam_client)
      }
      Err(e) => Err(Error::from_reason(format!("Failed to init: {:?}", e))),
    }
  }

  fn start_pumping_callbacks_internal(client: Arc<Client>) {
    tokio::spawn(async move {
      let mut intv = tokio::time::interval(tokio::time::Duration::from_millis(20));
      loop {
        intv.tick().await;
        client.run_callbacks();
      }
    });
  }

  #[napi(getter)]
  pub fn workshop(&self) -> UGCClient {
    return UGCClient::new(self.client.clone());
  }

  // Get information about the current user
  #[napi]
  pub fn get_current_user(&self) -> Result<UserInfo> {
    // Get the user interface
    let user = self.client.user();
    // Get the steam ID
    let steam_id = user.steam_id();

    let account_id = steam_id.account_id();
    
    // Get the user's name from FriendsClient
    let name = self.friends.get_persona_name()?;
    // Get the current persona state as an enum
    let state = self.friends.get_persona_state()?;
    
    let level = user.level();
    let logged_on = user.logged_on();

    Ok(UserInfo {
      steam_id: steam_id.raw() as f64,
      account_id: account_id.raw() as u32,
      name,
      state,
      level,
      logged_on,
    })
  }

  // // Get workshop item details asynchronously
  // #[napi]
  // pub fn get_item(&self, item_id: f64) -> AsyncTask<WorkshopQueryItemTask> {
  //   AsyncTask::new(WorkshopQueryItemTask {
  //     client: self.client.clone(),
  //     item_id,
  //   })
  // }

  // Get subscribed workshop items
  #[napi]
  pub fn get_subscriptions(&self) -> Result<Vec<f64>> {
    self.ugc.get_subscriptions()
  }
  // ----- FRIENDS FACADE METHODS -----

  // Get the number of friends
  #[napi]
  pub fn get_friend_count(&self, flags: Option<Vec<FriendRelationshipEnum>>) -> Result<u32> {
    self.friends.get_friend_count(flags)
  }

  // Get the list of friends
  #[napi]
  pub fn get_friends(&self, flags: Option<Vec<FriendRelationshipEnum>>) -> Result<Vec<FriendInfo>> {
    self.friends.get_friends(flags)
  }

  // Get the persona name of a friend
  #[napi]
  pub fn get_friend_persona_name(&self, steam_id: f64) -> Result<String> {
    self.friends.get_friend_persona_name(steam_id)
  }

  // Get the persona state of a friend
  #[napi]
  pub fn get_friend_persona_state(&self, steam_id: f64) -> Result<PersonaStateEnum> {
    self.friends.get_friend_persona_state(steam_id)
  }

  // Get the relationship with a friend
  #[napi]
  pub fn get_friend_relationship(&self, steam_id: f64) -> Result<FriendRelationshipEnum> {
    self.friends.get_friend_relationship(steam_id)
  }

  // Get the game played by a friend
  #[napi]
  pub fn get_friend_game_played(&self, steam_id: f64) -> Result<Option<String>> {
    self.friends.get_friend_game_played(steam_id)
  }

  // Set the persona name
  #[napi]
  pub fn set_persona_name(&self, name: String) -> Result<()> {
    self.friends.set_persona_name(name)
  }

  // Set the persona state
  #[napi]
  pub fn set_persona_state(&self, state: PersonaStateEnum) -> Result<()> {
    self.friends.set_persona_state(state)
  }

  // Get the current persona name
  #[napi]
  pub fn get_persona_name(&self) -> Result<String> {
    self.friends.get_persona_name()
  }

  // Get the current persona state
  #[napi]
  pub fn get_persona_state(&self) -> Result<PersonaStateEnum> {
    self.friends.get_persona_state()
  }

  // Add a friend
  #[napi]
  pub async fn add_friend(&self, steam_id: f64) -> Result<bool> {
    self.friends.add_friend(steam_id).await
  }

  // Remove a friend
  #[napi]
  pub fn remove_friend(&self, steam_id: f64) -> Result<bool> {
    self.friends.remove_friend(steam_id)
  }

  // Check if a user is a friend
  #[napi]
  pub fn has_friend(&self, steam_id: f64, flags: Option<Vec<FriendRelationshipEnum>>) -> Result<bool> {
    self.friends.has_friend(steam_id, flags)
  }

  // Request user information
  #[napi]
  pub fn request_user_information(&self, steam_id: f64, name_only: bool) -> Result<bool> {
    self.friends.request_user_information(steam_id, name_only)
  }

  // Set rich presence
  #[napi]
  pub fn set_rich_presence(&self, key: String, value: String) -> Result<bool> {
    self.friends.set_rich_presence(key, value)
  }

  // Clear rich presence
  #[napi]
  pub fn clear_rich_presence(&self) -> Result<()> {
    self.friends.clear_rich_presence()
  }

  // Get rich presence value
  #[napi]
  pub fn get_rich_presence(&self, key: String) -> Result<Option<String>> {
    self.friends.get_rich_presence(key)
  }

  // Get friend's rich presence value
  #[napi]
  pub fn get_friend_rich_presence(&self, steam_id: f64, key: String) -> Result<Option<String>> {
    self.friends.get_friend_rich_presence(steam_id, key)
  }

  // Get all rich presence keys
  #[napi]
  pub fn get_rich_presence_keys(&self) -> Result<Vec<String>> {
    self.friends.get_rich_presence_keys()
  }

  // Get all rich presence values
  #[napi]
  pub fn get_all_rich_presence(&self) -> Result<Vec<RichPresenceInfo>> {
    self.friends.get_all_rich_presence()
  }

  // Invite user to game
  #[napi]
  pub fn invite_user_to_game(&self, steam_id: f64, connect_string: String) -> Result<bool> {
    self.friends.invite_user_to_game(steam_id, connect_string)
  }
}

impl Drop for SteamClient {
  fn drop(&mut self) {
  }
}
