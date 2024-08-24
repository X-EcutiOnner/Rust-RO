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
// NV_BASIC
pub struct BasicSkill {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for BasicSkill {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        1
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
        9
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
    fn is_passive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {
        Some(self)
    }
}
impl PassiveSkillBase for BasicSkill {
}
// NV_FIRSTAID
pub struct FirstAid {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FirstAid {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        142
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
       3
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
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 3 { Ok(3) } else {Err(())}
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
impl PassiveSkillBase for FirstAid {
}
// NV_TRICKDEAD
pub struct PlayDead {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for PlayDead {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        143
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
       5
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
        if status.sp() > 5 { Ok(5) } else {Err(())}
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
impl SupportiveSkillBase for PlayDead {
}
