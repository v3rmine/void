use serde::{Serialize, Deserialize};

use super::Platform;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: String,
    pub anonymous: bool,
    pub verification: bool,
    pub ingame_name: String,
    pub check_code: String,
    pub role: UserRole,
    pub patreon_profile: PatreonProfile,
    pub platform: Platform,
    pub region: String,
    pub banned: bool,
    pub ban_reason: String,
    pub avatar: String,
    pub background: String,
    pub linked_accounts: LinkedAccounts,
    pub has_email: bool,
    pub written_reviews: u32,
    pub unread_messages: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Anonymous,
    User,
    Moderator,
    Admin    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatreonProfile {
    pub patreon_founder: bool,
    pub subscription: bool,
    pub patreon_badge: PatreonBadge,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatreonBadge {
    Bronze,
    Gold,
    Silver,
    Platinum   
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccounts {
    pub steam_profile: bool,
    pub patreon_profile: bool,
    pub xbox_profile: bool,
}