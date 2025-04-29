use steamworks::{FileType, UGCType, UserList, UserListOrder};

// Workshop item details
#[napi(object)]
pub struct WorkshopItemDetails {
  pub item_id: f64,
  pub title: String,
  pub description: String,
  pub owner_id: f64,
  pub time_created: u32,
  pub time_updated: u32,
  pub time_added_to_user_list: u32,
  pub visibility: u32,
  pub banned: bool,
  pub accepted_for_use: bool,
  pub tags_truncated: bool,
  pub tags: Vec<String>,
  pub file_size: u32,
  pub url: String,
  pub num_upvotes: u32,
  pub num_downvotes: u32,
  pub score: f64,
  pub num_children: u32,
}

#[napi]
pub fn item_state_to_string(state: WorkshopItemStateEnum) -> &'static str {
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
pub enum WorkshopItemVisibility {
  Public = 0,
  FriendsOnly = 1,
  Private = 2,
  Unlisted = 3,
}

// Workshop file type
#[napi]
pub enum WorkshopFileType {
  Community = 0,
  Microtransaction = 1,
  Collection = 2,
  Art = 3,
  Video = 4,
  Screenshot = 5,
  Game = 6,
  Software = 7,
  Concept = 8,
  WebGuide = 9,
  IntegratedGuide = 10,
  Merch = 11,
  ControllerBinding = 12,
  SteamworksAccessInvite = 13,
  SteamVideo = 14,
  GameManagedItem = 15,
}

impl WorkshopFileType {
    pub const fn to_file_type(&self) -> FileType {
        match *self {
            WorkshopFileType::Community => FileType::Community,
            WorkshopFileType::Microtransaction => FileType::Microtransaction,
            WorkshopFileType::Collection => FileType::Collection,
            WorkshopFileType::Art => FileType::Art,
            WorkshopFileType::Video => FileType::Video,
            WorkshopFileType::Screenshot => FileType::Screenshot,
            WorkshopFileType::Game => FileType::Game,
            WorkshopFileType::Software => FileType::Software,
            WorkshopFileType::Concept => FileType::Concept,
            WorkshopFileType::WebGuide => FileType::WebGuide,
            WorkshopFileType::IntegratedGuide => FileType::IntegratedGuide,
            WorkshopFileType::Merch => FileType::Merch,
            WorkshopFileType::ControllerBinding => FileType::ControllerBinding,
            WorkshopFileType::SteamworksAccessInvite => FileType::SteamworksAccessInvite,
            WorkshopFileType::SteamVideo => FileType::SteamVideo,
            WorkshopFileType::GameManagedItem => FileType::GameManagedItem,
        }
    }
}

// Workshop query types
#[napi]
pub enum WorkshopQueryType {
  RankedByVote = 0,
  RankedByPublicationDate = 1,
  AcceptedForGameRankedByAcceptanceDate = 2,
  RankedByTrend = 3,
  FavoritedByFriendsRankedByPublicationDate = 4,
  CreatedByFriendsRankedByPublicationDate = 5,
  RankedByNumTimesReported = 6,
  CreatedByFollowedUsersRankedByPublicationDate = 7,
  NotYetRated = 8,
  RankedByTotalVotesAsc = 9,
  RankedByVotesUp = 10,
  RankedByTextSearch = 11,
  RankedByTotalUniqueSubscriptions = 12,
  RankedByPlaytimeTrend = 13,
  RankedByTotalPlaytime = 14,
  RankedByAveragePlaytimeTrend = 15,
  RankedByLifetimeAveragePlaytime = 16,
  RankedByPlaytimeSessionsTrend = 17,
  RankedByLifetimePlaytimeSessions = 18,
}

// Workshop UGC types
#[napi]
pub enum WorkshopUGCType {
  Items = 0,
  ItemsMtx = 1,
  ItemsReadyToUse = 2,
  Collections = 3,
  Artwork = 4,
  Videos = 5,
  Screenshots = 6,
  AllGuides = 7,
  WebGuides = 8,
  IntegratedGuides = 9,
  UsableInGame = 10,
  ControllerBindings = 11,
  GameManagedItems = 12,
  All = 13,
}

impl WorkshopUGCType {
    pub const fn to_ugc_type(&self) -> UGCType {
        match *self {
            WorkshopUGCType::Items => UGCType::Items,
            WorkshopUGCType::ItemsMtx => UGCType::ItemsMtx,
            WorkshopUGCType::ItemsReadyToUse => UGCType::ItemsReadyToUse,
            WorkshopUGCType::Collections => UGCType::Collections,
            WorkshopUGCType::Artwork => UGCType::Artwork,
            WorkshopUGCType::Videos => UGCType::Videos,
            WorkshopUGCType::Screenshots => UGCType::Screenshots,
            WorkshopUGCType::AllGuides => UGCType::AllGuides,
            WorkshopUGCType::WebGuides => UGCType::WebGuides,
            WorkshopUGCType::IntegratedGuides => UGCType::IntegratedGuides,
            WorkshopUGCType::UsableInGame => UGCType::UsableInGame,
            WorkshopUGCType::ControllerBindings => UGCType::ControllerBindings,
            WorkshopUGCType::GameManagedItems => UGCType::GameManagedItems,
            WorkshopUGCType::All => UGCType::All,
        }
    }
}

// User workshop list type
#[napi]
pub enum WorkshopUserListType {
  Published = 0,
  VotedOn = 1,
  VotedUp = 2,
  VotedDown = 3,
  WillVoteLater = 4,
  Favorited = 5,
  Subscribed = 6,
  UsedOrPlayed = 7,
  Followed = 8,
}

impl WorkshopUserListType {
    pub const fn to_user_list(&self) -> UserList {
        match *self {
            WorkshopUserListType::Published => UserList::Published,
            WorkshopUserListType::VotedOn => UserList::VotedOn,
            WorkshopUserListType::VotedUp => UserList::VotedUp,
            WorkshopUserListType::VotedDown => UserList::VotedDown,
            WorkshopUserListType::WillVoteLater => UserList::WillVoteLater,
            WorkshopUserListType::Favorited => UserList::Favorited,
            WorkshopUserListType::Subscribed => UserList::Subscribed,
            WorkshopUserListType::UsedOrPlayed => UserList::UsedOrPlayed,
            WorkshopUserListType::Followed => UserList::Followed,
        }
    }
}

// User workshop list sort order
#[napi]
pub enum WorkshopUserListOrder {
  CreationOrderDesc = 0,
  CreationOrderAsc = 1,
  TitleAsc = 2,
  LastUpdatedDesc = 3,
  SubscriptionDateDesc = 4,
  VoteScoreDesc = 5,
  ForModeration = 6,
}

impl WorkshopUserListOrder {
    pub const fn to_user_list_order(&self) -> UserListOrder {
        match *self {
            WorkshopUserListOrder::CreationOrderDesc => UserListOrder::CreationOrderDesc,
            WorkshopUserListOrder::CreationOrderAsc => UserListOrder::CreationOrderAsc,
            WorkshopUserListOrder::TitleAsc => UserListOrder::TitleAsc,
            WorkshopUserListOrder::LastUpdatedDesc => UserListOrder::LastUpdatedDesc,
            WorkshopUserListOrder::SubscriptionDateDesc => UserListOrder::SubscriptionDateDesc,
            WorkshopUserListOrder::VoteScoreDesc => UserListOrder::VoteScoreDesc,
            WorkshopUserListOrder::ForModeration => UserListOrder::ForModeration
        }
    }
}

// Workshop item state for bitflags
#[napi]
pub enum WorkshopItemStateEnum {
  None = 0,
  Subscribed = 1,
  LegacyItem = 2,
  Installed = 4,
  NeedsUpdate = 8,
  Downloading = 16,
  DownloadPending = 32,
}

// Workshop item update details
#[napi(object)]
pub struct WorkshopItemUpdateDetails {
  pub title: Option<String>,
  pub description: Option<String>,
  pub changelog: Option<String>,
  pub preview_path: Option<String>,
  pub content_path: Option<String>,
  pub tags: Option<Vec<String>>,
  pub visibility: Option<u32>, // 0=Public, 1=FriendsOnly, 2=Private
}

// Workshop item creation result
#[napi(object)]
pub struct WorkshopItemCreationResult {
  pub item_id: f64,
  pub needs_to_accept_agreement: bool,
}

// Workshop item installation info
#[napi(object)]
pub struct WorkshopItemInstallInfo {
  pub folder: String,
  pub size_on_disk: f64, // Using f64 instead of u64 for JS compatibility
  pub timestamp: u32,
}

// Download info
#[napi(object)]
pub struct DownloadInfo {
  pub current: f64, // Using f64 instead of u64 for JS compatibility
  pub total: f64,   // Using f64 instead of u64 for JS compatibility
}
