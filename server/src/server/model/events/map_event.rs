use crate::server::model::action::Damage;


use crate::server::model::map_item::{MapItem, MapItemSnapshot};

#[derive(Debug, PartialEq)]
pub enum MapEvent {
    UpdateMobsFov(Vec<MapItemSnapshot>),
    RemoveCharFromMap(u32),
    InsertCharToMap(MapItem),
    RemoveDroppedItemFromMap(u32),
    MobDamage(Damage),
    MobDeathClientNotification(MobLocation),
    MobDropItems(MobDropItems),
    CharDropItems(CharacterDropItems),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobLocation {
    pub mob_id: u32,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobDropItems {
    pub owner_id: u32,
    pub mob_id: i16,
    pub mob_x: u16,
    pub mob_y: u16
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterDropItems {
    pub owner_id: u32,
    pub char_x: u16,
    pub char_y: u16,
    pub item_id_amount: Vec<(u32, u16)>
}