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
// CR_TRUST
pub struct Faith {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Faith {
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
        248
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
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
// CR_AUTOGUARD
pub struct Guard {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Guard {
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
        249
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
        if self.level == 6 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
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
        if let Some(state) = state {
            // Shield
            if state & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
// CR_SHIELDCHARGE
pub struct Smite {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Smite {
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
        250
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 10 { Ok(10) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            // Shield
            if state & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
// CR_SHIELDBOOMERANG
pub struct ShieldBoomerang {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ShieldBoomerang {
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
        251
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 12 { Ok(12) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            // Shield
            if state & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
       700
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// CR_REFLECTSHIELD
pub struct ShieldReflect {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ShieldReflect {
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
        252
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 65 { return Ok(65) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 75 { return Ok(75) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
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
        if let Some(state) = state {
            // Shield
            if state & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
// CR_HOLYCROSS
pub struct HolyCross {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for HolyCross {
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
        253
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 11 { return Ok(11) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 13 { return Ok(13) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 10 {
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
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       -2
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// CR_GRANDCROSS
pub struct GrandCross {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for GrandCross {
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
        254
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 51 { return Ok(51) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 58 { return Ok(58) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 65 { return Ok(65) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 72 { return Ok(72) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 79 { return Ok(79) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 86 { return Ok(86) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 93 { return Ok(93) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 100 { return Ok(100) } else {return Err(())}
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
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
       3000
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
       1500
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// CR_DEVOTION
pub struct Sacrifice {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Sacrifice {
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
        255
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 25 { Ok(25) } else {Err(())}
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
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
       3000
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
// CR_PROVIDENCE
pub struct ResistantSouls {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ResistantSouls {
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
        256
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 30 { Ok(30) } else {Err(())}
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
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn base_cast_time(&self) -> u32 {
       3000
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
// CR_DEFENDER
pub struct DefendingAura {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for DefendingAura {
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
        257
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 30 { Ok(30) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            // Shield
            if state & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
       800
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// CR_SPEARQUICKEN
pub struct SpearQuicken {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for SpearQuicken {
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
        258
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 48 { return Ok(48) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 56 { return Ok(56) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 60 { return Ok(60) } else {return Err(())}
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
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 32 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
// CR_SHRINK
pub struct Shrink {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Shrink {
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
        1002
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 15 { Ok(15) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            // Shield
            if state & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<WearWeapon>) -> SkillRequirementResult<()> {
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
