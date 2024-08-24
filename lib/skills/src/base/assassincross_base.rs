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
// ASC_KATAR
pub struct AdvancedKatarMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AdvancedKatarMastery {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        376
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
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
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
impl PassiveSkillBase for AdvancedKatarMastery {
}
// ASC_EDP
pub struct EnchantDeadlyPoison {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for EnchantDeadlyPoison {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        378
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
        if self.level == 1 {
            return 60
        }
        if self.level == 2 {
            return 70
        }
        if self.level == 3 {
            return 80
        }
        if self.level == 4 {
            return 90
        }
        if self.level == 5 {
            return 100
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
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 678, name_english: "Poison_Bottle".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 678 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
}
impl SupportiveSkillBase for EnchantDeadlyPoison {
    #[inline(always)]
    fn _bonuses(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(100), 2, tick, 40000),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(378, 3.0), 2, tick, 40000),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(-106), 2, tick, 45000),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(378, 3.5), 2, tick, 45000),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(-56), 2, tick, 50000),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(378, 4.0), 2, tick, 50000),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(-6), 2, tick, 55000),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(378, 4.5), 2, tick, 55000),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(44), 2, tick, 60000),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(378, 5.0), 2, tick, 60000),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn _bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::default()
    }
}
// ASC_BREAKER
pub struct SoulDestroyer {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SoulDestroyer {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        379
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
        10
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 20
        }
        if self.level == 2 {
            return 20
        }
        if self.level == 3 {
            return 20
        }
        if self.level == 4 {
            return 20
        }
        if self.level == 5 {
            return 20
        }
        if self.level == 6 {
            return 30
        }
        if self.level == 7 {
            return 30
        }
        if self.level == 8 {
            return 30
        }
        if self.level == 9 {
            return 30
        }
        if self.level == 10 {
            return 30
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        true
    }
    fn _is_physical(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       700
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1400
        }
        if self.level == 4 {
            return 1600
        }
        if self.level == 5 {
            return 1800
        }
        if self.level == 6 {
            return 2000
        }
        if self.level == 7 {
            return 2200
        }
        if self.level == 8 {
            return 2400
        }
        if self.level == 9 {
            return 2600
        }
        if self.level == 10 {
            return 2800
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1400
        }
        if self.level == 4 {
            return 1600
        }
        if self.level == 5 {
            return 1800
        }
        if self.level == 6 {
            return 2000
        }
        if self.level == 7 {
            return 2200
        }
        if self.level == 8 {
            return 2400
        }
        if self.level == 9 {
            return 2600
        }
        if self.level == 10 {
            return 2800
        }
        0
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for SoulDestroyer {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.000)
        }
        if self.level == 2 {
            return Some(2.000)
        }
        if self.level == 3 {
            return Some(3.000)
        }
        if self.level == 4 {
            return Some(4.000)
        }
        if self.level == 5 {
            return Some(5.000)
        }
        if self.level == 6 {
            return Some(6.000)
        }
        if self.level == 7 {
            return Some(7.000)
        }
        if self.level == 8 {
            return Some(8.000)
        }
        if self.level == 9 {
            return Some(9.000)
        }
        if self.level == 10 {
            return Some(10.000)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
}
// ASC_METEORASSAULT
pub struct MeteorAssault {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MeteorAssault {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        406
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
        if self.level == 1 {
            return 10
        }
        if self.level == 2 {
            return 12
        }
        if self.level == 3 {
            return 14
        }
        if self.level == 4 {
            return 16
        }
        if self.level == 5 {
            return 18
        }
        if self.level == 6 {
            return 20
        }
        if self.level == 7 {
            return 22
        }
        if self.level == 8 {
            return 24
        }
        if self.level == 9 {
            return 26
        }
        if self.level == 10 {
            return 28
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
        true
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       500
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       500
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for MeteorAssault {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(0.800)
        }
        if self.level == 2 {
            return Some(1.200)
        }
        if self.level == 3 {
            return Some(1.600)
        }
        if self.level == 4 {
            return Some(2.000)
        }
        if self.level == 5 {
            return Some(2.400)
        }
        if self.level == 6 {
            return Some(2.800)
        }
        if self.level == 7 {
            return Some(3.200)
        }
        if self.level == 8 {
            return Some(3.600)
        }
        if self.level == 9 {
            return Some(4.000)
        }
        if self.level == 10 {
            return Some(4.400)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
}
// ASC_CDP
pub struct CreateDeadlyPoison {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CreateDeadlyPoison {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        407
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
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       50
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
        if status.sp() > 50 { Ok(50) } else {Err(())}
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       500
    }
}
