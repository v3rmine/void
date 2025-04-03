use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::InventorySlotSchema;

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterSchema {
    pub name: String,
    pub skin: String,
    pub level: u32,
    pub xp: u32,
    pub max_xp: u32,
    pub total_xp: u32,
    pub gold: u32,
    pub mining_level: u32,
    pub mining_xp: u32,
    pub mining_max_xp: u32,
    pub attack_level: u32,
    pub woodcutting_level: u32,
    pub woodcutting_xp: u32,
    pub woodcutting_max_xp: u32,
    pub fishing_level: u32,
    pub fishing_xp: u32,
    pub fishing_max_xp: u32,
    pub weaponcrafting_level: u32,
    pub weaponcrafting_xp: u32,
    pub weaponcrafting_max_xp: u32,
    pub gearcrafting_level: u32,
    pub gearcrafting_xp: u32,
    pub gearcrafting_max_xp: u32,
    pub jewelrycrafting_level: u32,
    pub jewelrycrafting_xp: u32,
    pub jewelrycrafting_max_xp: u32,
    pub cooking_level: u32,
    pub cooking_xp: u32,
    pub cooking_max_xp: u32,
    pub hp: u32,
    pub haste: u32,
    pub attack_fire: u32,
    pub attack_earth: u32,
    pub attack_water: u32,
    pub attack_air: u32,
    pub dmg_fire: u32,
    pub dmg_earth: u32,
    pub dmg_water: u32,
    pub dmg_air: u32,
    pub res_fire: u32,
    pub res_earth: u32,
    pub res_water: u32,
    pub res_air: u32,
    pub x: u32,
    pub y: u32,
    pub cooldown: u32,
    pub cooldown_expiration: DateTime<Utc>,
    pub weapon_slot: String,
    pub shield_slot: String,
    pub helmet_slot: String,
    pub body_armor_slot: String,
    pub leg_armor_slot: String,
    pub boots_slot: String,
    pub ring1_slot: String,
    pub ring2_slot: String,
    pub amulet_slot: String,
    pub artifact1_slot: String,
    pub artifact2_slot: String,
    pub artifact3_slot: String,
    pub consumable1_slot: String,
    pub consumable1_slot_quantity: u32,
    pub consumable2_slot: String,
    pub consumable2_slot_quantity: u32,
    pub task: String,
    pub task_type: String,
    pub task_progress: u32,
    pub task_total: u32,
    pub inventory_max_items: u32,
    pub inventory: Vec<InventorySlotSchema>,
}
