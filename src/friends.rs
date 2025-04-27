use std::sync::Arc;

use napi::bindgen_prelude::*;
use steamworks::{FriendFlags, Friend};

#[napi(object)]
pub struct FriendInfo {
    pub steam_id: f64,
    pub name: String,
    pub state: u32,
    pub relationship: u32,
    pub game_played: Option<String>,
}

#[napi(string_enum)]
pub enum FriendRelationshipEnum {
    None,
    Blocked,
    RequestRecipient,
    Friend,
    RequestInitiator,
    Ignored,
    IgnoredFriend,
}

#[napi(string_enum)]
pub enum PersonaStateEnum {
    Offline,
    Online,
    Busy,
    Away,
    Snooze,
    LookingToTrade,
    LookingToPlay,
    Invisible,
}

#[napi(object)]
pub struct RichPresenceInfo {
    pub key: String,
    pub value: String,
}

pub struct FriendsClient {
    client: Arc<steamworks::Client>,
}

impl FriendsClient {
    pub fn new(client: Arc<steamworks::Client>) -> Self {
        FriendsClient { client }
    }

    pub fn get_friend_count(&self, flags: Option<Vec<FriendRelationshipEnum>>) -> Result<u32> {
        let friends = self.client.friends();
        
        let flags = match flags {
            Some(enum_flags) => {
                let mut friend_flags = FriendFlags::empty();
                for flag in enum_flags {
                    match flag {
                        FriendRelationshipEnum::None => {},
                        FriendRelationshipEnum::Blocked => friend_flags |= FriendFlags::BLOCKED,
                        FriendRelationshipEnum::RequestRecipient => friend_flags |= FriendFlags::FRIENDSHIP_REQUESTED,
                        FriendRelationshipEnum::Friend => friend_flags |= FriendFlags::IMMEDIATE,
                        FriendRelationshipEnum::RequestInitiator => friend_flags |= FriendFlags::REQUESTING_FRIENDSHIP,
                        FriendRelationshipEnum::Ignored => friend_flags |= FriendFlags::IGNORED,
                        FriendRelationshipEnum::IgnoredFriend => friend_flags |= FriendFlags::IGNORED_FRIEND,
                    }
                }
                friend_flags
            },
            None => FriendFlags::IMMEDIATE, // Default to immediate friends
        };
        
        // There's no get_friend_count in the API, we'll use get_friends().len() instead
        let count = friends.get_friends(flags).len();
        Ok(count as u32)
    }

    pub fn get_friends(&self, flags: Option<Vec<FriendRelationshipEnum>>) -> Result<Vec<FriendInfo>> {
        let friends = self.client.friends();
        
        let flags = match flags {
            Some(enum_flags) => {
                let mut friend_flags = FriendFlags::empty();
                for flag in enum_flags {
                    match flag {
                        FriendRelationshipEnum::None => {},
                        FriendRelationshipEnum::Blocked => friend_flags |= FriendFlags::BLOCKED,
                        FriendRelationshipEnum::RequestRecipient => friend_flags |= FriendFlags::FRIENDSHIP_REQUESTED,
                        FriendRelationshipEnum::Friend => friend_flags |= FriendFlags::IMMEDIATE,
                        FriendRelationshipEnum::RequestInitiator => friend_flags |= FriendFlags::REQUESTING_FRIENDSHIP,
                        FriendRelationshipEnum::Ignored => friend_flags |= FriendFlags::IGNORED,
                        FriendRelationshipEnum::IgnoredFriend => friend_flags |= FriendFlags::IGNORED_FRIEND,
                    }
                }
                friend_flags
            },
            None => FriendFlags::IMMEDIATE, // Default to immediate friends
        };
        
        let mut friend_list = Vec::new();
        let steam_friends = friends.get_friends(flags);
        
        for friend in steam_friends {
            let friend_id = friend.id().raw() as f64;
            let name = friend.name();
            let state = self.map_persona_state(friend.state());
            
            // Use a reference to avoid moving the friend
            let relationship = self.map_friend_relationship(&friend);
            
            // Get game played if available
            let game_played = if let Some(game) = friend.game_played() {
                Some(format!("{}", game.game.app_id().0))
            } else {
                None
            };
            
            friend_list.push(FriendInfo {
                steam_id: friend_id,
                name,
                state,
                relationship,
                game_played,
            });
        }
        
        Ok(friend_list)
    }
    
    pub fn get_friend_persona_name(&self, steam_id: f64) -> Result<String> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        let friend = friends.get_friend(steam_id);
        Ok(friend.name())
    }
    
    pub fn get_friend_persona_state(&self, steam_id: f64) -> Result<PersonaStateEnum> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        let friend = friends.get_friend(steam_id);
        Ok(self.to_persona_state_enum(friend.state()))
    }
    
    pub fn get_friend_relationship(&self, steam_id: f64) -> Result<FriendRelationshipEnum> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        let friend = friends.get_friend(steam_id);
        Ok(self.get_friend_relationship_enum(&friend))
    }
    
    pub fn get_friend_game_played(&self, steam_id: f64) -> Result<Option<String>> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        let friend = friends.get_friend(steam_id);
        
        if let Some(game) = friend.game_played() {
            Ok(Some(format!("{}", game.game.app_id().0)))
        } else {
            Ok(None)
        }
    }
    
    pub fn set_persona_name(&self, _name: String) -> Result<()> {
        // The steamworks-rs library doesn't seem to have a public method for this
        // Let's leave it as a stub that does nothing
        Ok(())
    }
    
    pub fn set_persona_state(&self, _state: PersonaStateEnum) -> Result<()> {
        // The steamworks-rs library doesn't provide a way to set persona state
        // The Steam API does have this functionality but it's not exposed in the Rust library
        Ok(())
    }
    
    pub fn get_persona_name(&self) -> Result<String> {
        let friends = self.client.friends();
        Ok(friends.name())
    }
    
    pub fn get_persona_state(&self) -> Result<PersonaStateEnum> {
        // The steamworks-rs library doesn't provide a way to get your own persona state
        // Return Online as default
        Ok(PersonaStateEnum::Online)
    }
    
    pub async fn add_friend(&self, steam_id: f64) -> Result<bool> {
        // The steamworks-rs library doesn't expose this method as a callback
        // This is a placeholder for future implementation
        let _ = steam_id;
        // For now, just return a stub value
        Ok(false)
    }
    
    pub fn remove_friend(&self, _steam_id: f64) -> Result<bool> {
        // steamworks-rs doesn't expose this method directly
        // For now, just return a stub value
        Ok(false)
    }
    
    pub fn has_friend(&self, steam_id: f64, flags: Option<Vec<FriendRelationshipEnum>>) -> Result<bool> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        let friend = friends.get_friend(steam_id);
        
        let flags = match flags {
            Some(enum_flags) => {
                let mut friend_flags = FriendFlags::empty();
                for flag in enum_flags {
                    match flag {
                        FriendRelationshipEnum::None => {},
                        FriendRelationshipEnum::Blocked => friend_flags |= FriendFlags::BLOCKED,
                        FriendRelationshipEnum::RequestRecipient => friend_flags |= FriendFlags::FRIENDSHIP_REQUESTED,
                        FriendRelationshipEnum::Friend => friend_flags |= FriendFlags::IMMEDIATE,
                        FriendRelationshipEnum::RequestInitiator => friend_flags |= FriendFlags::REQUESTING_FRIENDSHIP,
                        FriendRelationshipEnum::Ignored => friend_flags |= FriendFlags::IGNORED,
                        FriendRelationshipEnum::IgnoredFriend => friend_flags |= FriendFlags::IGNORED_FRIEND,
                    }
                }
                friend_flags
            },
            None => FriendFlags::IMMEDIATE, // Default to immediate friends
        };
        
        Ok(friend.has_friend(flags))
    }
    
    pub fn request_user_information(&self, steam_id: f64, name_only: bool) -> Result<bool> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        Ok(friends.request_user_information(steam_id, name_only))
    }
    
    pub fn set_rich_presence(&self, key: String, value: String) -> Result<bool> {
        let friends = self.client.friends();
        Ok(friends.set_rich_presence(&key, Some(&value)))
    }
    
    pub fn clear_rich_presence(&self) -> Result<()> {
        let friends = self.client.friends();
        friends.clear_rich_presence();
        Ok(())
    }
    
    pub fn get_rich_presence(&self, _key: String) -> Result<Option<String>> {
        // The steamworks-rs library doesn't provide a way to get rich presence
        Ok(None)
    }
    
    pub fn get_friend_rich_presence(&self, _steam_id: f64, _key: String) -> Result<Option<String>> {
        // The steamworks-rs library doesn't provide a way to get friend's rich presence
        Ok(None)
    }
    
    pub fn get_rich_presence_keys(&self) -> Result<Vec<String>> {
        // The steamworks-rs library doesn't provide a way to get rich presence keys
        Ok(Vec::new())
    }
    
    pub fn get_all_rich_presence(&self) -> Result<Vec<RichPresenceInfo>> {
        // The steamworks-rs library doesn't provide a way to get all rich presence
        Ok(Vec::new())
    }
    
    pub fn invite_user_to_game(&self, steam_id: f64, connect_string: String) -> Result<bool> {
        let friends = self.client.friends();
        let steam_id = steamworks::SteamId::from_raw(steam_id as u64);
        let friend = friends.get_friend(steam_id);
        friend.invite_user_to_game(&connect_string);
        Ok(true)
    }
    
    // Helper function to map steamworks::FriendState to u32
    fn map_persona_state(&self, state: steamworks::FriendState) -> u32 {
        match state {
            steamworks::FriendState::Offline => 0,
            steamworks::FriendState::Online => 1,
            steamworks::FriendState::Busy => 2,
            steamworks::FriendState::Away => 3,
            steamworks::FriendState::Snooze => 4,
            steamworks::FriendState::LookingToTrade => 5,
            steamworks::FriendState::LookingToPlay => 6,
        }
    }
    
    // Helper function to map Friend to relationship u32
    fn map_friend_relationship(&self, friend: &Friend<steamworks::ClientManager>) -> u32 {
        if friend.has_friend(FriendFlags::BLOCKED) {
            return 1; // Blocked
        }
        if friend.has_friend(FriendFlags::FRIENDSHIP_REQUESTED) {
            return 2; // RequestRecipient
        }
        if friend.has_friend(FriendFlags::IMMEDIATE) {
            return 3; // Friend
        }
        if friend.has_friend(FriendFlags::REQUESTING_FRIENDSHIP) {
            return 4; // RequestInitiator
        }
        if friend.has_friend(FriendFlags::IGNORED) {
            return 5; // Ignored
        }
        if friend.has_friend(FriendFlags::IGNORED_FRIEND) {
            return 6; // IgnoredFriend
        }
        0 // None
    }
    
    // Helper function to convert steamworks::FriendState to PersonaStateEnum
    fn to_persona_state_enum(&self, state: steamworks::FriendState) -> PersonaStateEnum {
        match state {
            steamworks::FriendState::Offline => PersonaStateEnum::Offline,
            steamworks::FriendState::Online => PersonaStateEnum::Online,
            steamworks::FriendState::Busy => PersonaStateEnum::Busy,
            steamworks::FriendState::Away => PersonaStateEnum::Away,
            steamworks::FriendState::Snooze => PersonaStateEnum::Snooze,
            steamworks::FriendState::LookingToTrade => PersonaStateEnum::LookingToTrade,
            steamworks::FriendState::LookingToPlay => PersonaStateEnum::LookingToPlay,
        }
    }
    
    // Helper function to determine Friend's relationship enum
    fn get_friend_relationship_enum(&self, friend: &Friend<steamworks::ClientManager>) -> FriendRelationshipEnum {
        if friend.has_friend(FriendFlags::BLOCKED) {
            return FriendRelationshipEnum::Blocked;
        }
        if friend.has_friend(FriendFlags::FRIENDSHIP_REQUESTED) {
            return FriendRelationshipEnum::RequestRecipient;
        }
        if friend.has_friend(FriendFlags::IMMEDIATE) {
            return FriendRelationshipEnum::Friend;
        }
        if friend.has_friend(FriendFlags::REQUESTING_FRIENDSHIP) {
            return FriendRelationshipEnum::RequestInitiator;
        }
        if friend.has_friend(FriendFlags::IGNORED) {
            return FriendRelationshipEnum::Ignored;
        }
        if friend.has_friend(FriendFlags::IGNORED_FRIEND) {
            return FriendRelationshipEnum::IgnoredFriend;
        }
        FriendRelationshipEnum::None
    }
}