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
// HW_SOULDRAIN - Soul Drain
pub struct SoulDrain {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SoulDrain {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        364
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
        true
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![]);
        }
        TemporaryStatusBonuses::default()
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
impl OffensiveSkillBase for SoulDrain {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Neutral
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
// HW_MAGICCRASHER - Stave Crasher
pub struct StaveCrasher {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for StaveCrasher {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        365
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
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       8
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        true
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 8 { Ok(8) } else {Err(())}
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       300
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       300
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
impl OffensiveSkillBase for StaveCrasher {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_matk(&self) -> Option<f32> {
       Some(1.000)
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
// HW_MAGICPOWER - Mystical Amplification
pub struct MysticalAmplification {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MysticalAmplification {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        366
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
            return 14
        }
        if self.level == 2 {
            return 18
        }
        if self.level == 3 {
            return 22
        }
        if self.level == 4 {
            return 26
        }
        if self.level == 5 {
            return 30
        }
        if self.level == 6 {
            return 34
        }
        if self.level == 7 {
            return 38
        }
        if self.level == 8 {
            return 42
        }
        if self.level == 9 {
            return 46
        }
        if self.level == 10 {
            return 50
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
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       700
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(5), 2, tick, 30000),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(10), 2, tick, 30000),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(15), 2, tick, 30000),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(20), 2, tick, 30000),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(25), 2, tick, 30000),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(30), 2, tick, 30000),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(35), 2, tick, 30000),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(40), 2, tick, 30000),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(45), 2, tick, 30000),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::MatkPercentage(50), 2, tick, 30000),]);
        }
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
impl SupportiveSkillBase for MysticalAmplification {
}
// HW_NAPALMVULCAN - Napalm Vulcan
pub struct NapalmVulcan {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for NapalmVulcan {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        400
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
            return 10
        }
        if self.level == 2 {
            return 25
        }
        if self.level == 3 {
            return 40
        }
        if self.level == 4 {
            return 55
        }
        if self.level == 5 {
            return 70
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
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
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
impl OffensiveSkillBase for NapalmVulcan {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        if self.level == 1 {
            return 1
        }
        if self.level == 2 {
            return 2
        }
        if self.level == 3 {
            return 3
        }
        if self.level == 4 {
            return 4
        }
        if self.level == 5 {
            return 5
        }
        0
    }
    #[inline(always)]
    fn _dmg_matk(&self) -> Option<f32> {
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
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Ghost
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        let mut effects = Vec::with_capacity(1);
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 5 {
                effects.push(StatusEffect::Curse);
            }
        }
        if self.level == 2 {
            if chance <= 10 {
                effects.push(StatusEffect::Curse);
            }
        }
        if self.level == 3 {
            if chance <= 15 {
                effects.push(StatusEffect::Curse);
            }
        }
        if self.level == 4 {
            if chance <= 20 {
                effects.push(StatusEffect::Curse);
            }
        }
        if self.level == 5 {
            if chance <= 25 {
                effects.push(StatusEffect::Curse);
            }
        }
        effects
    }
}
// HW_GANBANTEIN - Ganbantein
pub struct Ganbantein {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Ganbantein {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        483
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
       18
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       40
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 40 { Ok(40) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 715, name_english: "Yellow_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 715 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       3000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for Ganbantein {
}
// HW_GRAVITATION - Gravitation Field
pub struct GravitationField {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for GravitationField {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        484
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
       18
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
            return 40
        }
        if self.level == 3 {
            return 60
        }
        if self.level == 4 {
            return 80
        }
        if self.level == 5 {
            return 100
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
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
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       5000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-5.0), 2, tick, 5000),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-10.0), 2, tick, 6000),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-15.0), 2, tick, 7000),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-20.0), 2, tick, 8000),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-25.0), 2, tick, 9000),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for GravitationField {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Earth
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
impl GroundSkillBase for GravitationField {
}
