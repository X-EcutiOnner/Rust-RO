// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use models::enums::skill::*;
use models::enums::weapon::AmmoType;
use models::enums::element::Element::{*};

use models::item::WearWeapon;

use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;
use models::enums::weapon::WeaponType::{*};
use models::enums::bonus::{BonusType};
use models::enums::status::StatusEffect::{*};
use models::status_bonus::{TemporaryStatusBonus};
use models::enums::mob::MobRace::{*};

use crate::{*};

use crate::base::*;
use std::any::Any;
// HP_ASSUMPTIO
pub struct Assumptio {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Assumptio {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        361
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       9
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 20
        }
        if self.level == 2 {
            return 30
        }
        if self.level == 3 {
            return 40
        }
        if self.level == 4 {
            return 50
        }
        if self.level == 5 {
            return 60
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 1500
        }
        if self.level == 3 {
            return 2000
        }
        if self.level == 4 {
            return 2500
        }
        if self.level == 5 {
            return 3000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 1100
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1300
        }
        if self.level == 4 {
            return 1400
        }
        if self.level == 5 {
            return 1500
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 1100
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1300
        }
        if self.level == 4 {
            return 1400
        }
        if self.level == 5 {
            return 1500
        }
        0
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    fn _bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn _bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        16
    }
}
impl SupportiveSkillBase for Assumptio {
}
// HP_BASILICA
pub struct Basilica {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Basilica {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        362
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       4
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 80
        }
        if self.level == 2 {
            return 90
        }
        if self.level == 3 {
            return 100
        }
        if self.level == 4 {
            return 110
        }
        if self.level == 5 {
            return 120
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 110 { return Ok(110) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 120 { return Ok(120) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 715, name_english: "Yellow_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 716, name_english: "Red_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 523, name_english: "Holy_Water".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 715 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 716 && item.amount >= 1) {
            return Err(UseSkillFailure::RedGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 523 && item.amount >= 1) {
            return Err(UseSkillFailure::Holywater);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 5000
        }
        if self.level == 2 {
            return 6000
        }
        if self.level == 3 {
            return 7000
        }
        if self.level == 4 {
            return 8000
        }
        if self.level == 5 {
            return 9000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 4000
        }
        if self.level == 4 {
            return 5000
        }
        if self.level == 5 {
            return 6000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 4000
        }
        if self.level == 4 {
            return 5000
        }
        if self.level == 5 {
            return 6000
        }
        0
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    fn _bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn _bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        4
    }
}
impl SupportiveSkillBase for Basilica {
}
// HP_MEDITATIO
pub struct Meditatio {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Meditatio {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        363
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        10
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Passive
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(1), 0, 363),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(2), 0, 363),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(3), 0, 363),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(4), 0, 363),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(5), 0, 363),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(6), 0, 363),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(7), 0, 363),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(8), 0, 363),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(9), 0, 363),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MaxspPercentage(10), 0, 363),]);
        }
        TemporaryStatusBonuses::default()
    }
    fn _bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn _bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_passive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {
        Some(self)
    }
}
impl PassiveSkillBase for Meditatio {
}
// HP_MANARECHARGE
pub struct ManaRecharge {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ManaRecharge {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        481
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Passive
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    fn _bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn _bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
}
