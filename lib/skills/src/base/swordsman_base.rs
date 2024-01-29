// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;
use enums::element::Element;

use models::item::WearWeapon;

use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;

use crate::{*};

use crate::base::*;
use std::any::Any;
// SM_SWORD
pub struct SwordMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SwordMastery {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        2
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
        true
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
impl PassiveSkillBase for SwordMastery {
}
// SM_TWOHAND
pub struct TwoHandedSwordMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TwoHandedSwordMastery {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        3
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
        true
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
impl PassiveSkillBase for TwoHandedSwordMastery {
}
// SM_RECOVERY
pub struct IncreaseHpRecovery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for IncreaseHpRecovery {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        4
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
    fn is_passive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {
        Some(self)
    }
}
impl PassiveSkillBase for IncreaseHpRecovery {
}
// SM_BASH
pub struct Bash {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Bash {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        5
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
       -1
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
            return 8
        }
        if self.level == 2 {
            return 8
        }
        if self.level == 3 {
            return 8
        }
        if self.level == 4 {
            return 8
        }
        if self.level == 5 {
            return 8
        }
        if self.level == 6 {
            return 15
        }
        if self.level == 7 {
            return 15
        }
        if self.level == 8 {
            return 15
        }
        if self.level == 9 {
            return 15
        }
        if self.level == 10 {
            return 15
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Attack
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
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8386559 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            // Allow to use Fist
            Ok(())
        }
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
impl OffensiveSkillBase for Bash {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.300)
        }
        if self.level == 2 {
            return Some(1.600)
        }
        if self.level == 3 {
            return Some(1.900)
        }
        if self.level == 4 {
            return Some(2.200)
        }
        if self.level == 5 {
            return Some(2.500)
        }
        if self.level == 6 {
            return Some(2.800)
        }
        if self.level == 7 {
            return Some(3.100)
        }
        if self.level == 8 {
            return Some(3.400)
        }
        if self.level == 9 {
            return Some(3.700)
        }
        if self.level == 10 {
            return Some(4.000)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
}
// SM_PROVOKE
pub struct Provoke {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Provoke {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        6
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
            return 4
        }
        if self.level == 2 {
            return 5
        }
        if self.level == 3 {
            return 6
        }
        if self.level == 4 {
            return 7
        }
        if self.level == 5 {
            return 8
        }
        if self.level == 6 {
            return 9
        }
        if self.level == 7 {
            return 10
        }
        if self.level == 8 {
            return 11
        }
        if self.level == 9 {
            return 12
        }
        if self.level == 10 {
            return 13
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Attack
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
            if status.sp() >= 4 { return Ok(4) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 5 { return Ok(5) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 6 { return Ok(6) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 7 { return Ok(7) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 9 { return Ok(9) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 11 { return Ok(11) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 13 { return Ok(13) } else {return Err(())}
        }
        Err(())
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
impl OffensiveSkillBase for Provoke {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Neutral
    }
}
// SM_MAGNUM
pub struct MagnumBreak {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MagnumBreak {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        7
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
       30
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
        if status.sp() > 30 { Ok(30) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_hp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.hp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.hp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.hp() >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 4 {
            if status.hp() >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 5 {
            if status.hp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 6 {
            if status.hp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 7 {
            if status.hp() >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 8 {
            if status.hp() >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 9 {
            if status.hp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 10 {
            if status.hp() >= 16 { return Ok(16) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
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
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for MagnumBreak {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.200)
        }
        if self.level == 2 {
            return Some(1.400)
        }
        if self.level == 3 {
            return Some(1.600)
        }
        if self.level == 4 {
            return Some(1.800)
        }
        if self.level == 5 {
            return Some(2.000)
        }
        if self.level == 6 {
            return Some(2.200)
        }
        if self.level == 7 {
            return Some(2.400)
        }
        if self.level == 8 {
            return Some(2.600)
        }
        if self.level == 9 {
            return Some(2.800)
        }
        if self.level == 10 {
            return Some(3.000)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Fire
    }
}
impl SelfSkillBase for MagnumBreak {
}
// SM_ENDURE
pub struct Endure {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Endure {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        8
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
       10
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
        if status.sp() > 10 { Ok(10) } else {Err(())}
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for Endure {
}
// SM_MOVINGRECOVERY
pub struct MovingHpRecovery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MovingHpRecovery {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        144
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
impl PassiveSkillBase for MovingHpRecovery {
}
// SM_FATALBLOW
pub struct FatalBlow {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FatalBlow {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        145
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
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Passive
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        true
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
impl PassiveSkillBase for FatalBlow {
}
// SM_AUTOBERSERK
pub struct AutoBerserk {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AutoBerserk {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        146
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
       1
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
        if status.sp() > 1 { Ok(1) } else {Err(())}
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for AutoBerserk {
}
