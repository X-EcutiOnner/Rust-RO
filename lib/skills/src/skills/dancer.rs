// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;
use models::item::NormalInventoryItem;

use crate::{Skill, SkillRequirementResult, DelegateSkill};

use crate::skills::*;
// DC_DANCINGLESSON
pub struct DanceLessons {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for DanceLessons {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        323
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_THROWARROW
pub struct SlingingArrow {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for SlingingArrow {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        324
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 1 { return Ok(1) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 3 { return Ok(3) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 5 { return Ok(5) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 7 { return Ok(7) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 9 { return Ok(9) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        if let Some(ammo_and_amount) = character_ammo {
            if ammo_and_amount.1 >= 1 && (2 & ammo_and_amount.0.as_flag()) > 0 { Ok(1) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 16384 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
       1500
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_UGLYDANCE
pub struct HipShaker {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for HipShaker {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        325
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 16384 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_SCREAM
pub struct Dazzler {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Dazzler {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        326
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
       4000
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_HUMMING
pub struct FocusBallet {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for FocusBallet {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        327
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 24576 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_DONTFORGETME
pub struct SlowGrace {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for SlowGrace {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        328
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 31 { return Ok(31) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 43 { return Ok(43) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 55 { return Ok(55) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 24576 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_FORTUNEKISS
pub struct LadyLuck {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for LadyLuck {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        329
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 43 { return Ok(43) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 58 { return Ok(58) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 61 { return Ok(61) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 67 { return Ok(67) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 70 { return Ok(70) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 24576 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_SERVICEFORYOU
pub struct GypsysKiss {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for GypsysKiss {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        330
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 65 { return Ok(65) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 75 { return Ok(75) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 85 { return Ok(85) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 24576 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// DC_WINKCHARM
pub struct WinkofCharm {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for WinkofCharm {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, delegate: None, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn cast_time(&self) -> u32 {
        self.cast_time
    }
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn id(&self) -> u32 {
        1011
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 40 { Ok(40) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
       1000
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
