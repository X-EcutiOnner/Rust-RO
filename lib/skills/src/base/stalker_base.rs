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
// ST_CHASEWALK
pub struct Stealth {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Stealth {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        389
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
       10
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
        if status.sp() > 10 { Ok(10) } else {Err(())}
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1200
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-86), 2, tick, 10000),
                TemporaryStatusBonus::with_duration(BonusType::Str(1), 2, tick, 10000),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-81), 2, tick, 10000),
                TemporaryStatusBonus::with_duration(BonusType::Str(2), 2, tick, 10000),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-76), 2, tick, 10000),
                TemporaryStatusBonus::with_duration(BonusType::Str(4), 2, tick, 10000),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-71), 2, tick, 10000),
                TemporaryStatusBonus::with_duration(BonusType::Str(8), 2, tick, 10000),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-66), 2, tick, 10000),
                TemporaryStatusBonus::with_duration(BonusType::Str(16), 2, tick, 10000),]);
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
}
// ST_REJECTSWORD
pub struct CounterInstinct {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CounterInstinct {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        390
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
            return 10
        }
        if self.level == 2 {
            return 15
        }
        if self.level == 3 {
            return 20
        }
        if self.level == 4 {
            return 25
        }
        if self.level == 5 {
            return 30
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
            if status.sp() >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackReflectChancePercentage(15), 2, tick, 300000),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackReflectChancePercentage(30), 2, tick, 300000),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackReflectChancePercentage(45), 2, tick, 300000),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackReflectChancePercentage(60), 2, tick, 300000),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackReflectChancePercentage(75), 2, tick, 300000),]);
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
impl SupportiveSkillBase for CounterInstinct {
}
// ST_PRESERVE
pub struct Preserve {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Preserve {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        475
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
       30
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
        if status.sp() > 30 { Ok(30) } else {Err(())}
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
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
// ST_FULLSTRIP
pub struct DivestAll {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for DivestAll {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        476
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
       1
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
            return 22
        }
        if self.level == 2 {
            return 24
        }
        if self.level == 3 {
            return 26
        }
        if self.level == 4 {
            return 28
        }
        if self.level == 5 {
            return 30
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
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
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
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for DivestAll {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Neutral
    }
}
