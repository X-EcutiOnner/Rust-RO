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
// KN_SPEARMASTERY
pub struct SpearMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for SpearMastery {
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
        55
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
// KN_PIERCE
pub struct Pierce {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Pierce {
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
        56
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 7 { Ok(7) } else {Err(())}
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
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
       3
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// KN_BRANDISHSPEAR
pub struct BrandishSpear {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for BrandishSpear {
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
        57
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
            // Riding
            if state & 4 > 0 { Ok(()) } else { Err(()) }
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
        if let Some(character_weapon) = character_weapon {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
       700
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
// KN_SPEARSTAB
pub struct SpearStab {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for SpearStab {
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
        58
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 9 { Ok(9) } else {Err(())}
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
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
// KN_SPEARBOOMERANG
pub struct SpearBoomerang {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for SpearBoomerang {
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
        59
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
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
       1000
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// KN_TWOHANDQUICKEN
pub struct TwohandQuicken {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for TwohandQuicken {
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
        60
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 50 { return Ok(50) } else {return Err(())}
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
            if 8 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
// KN_AUTOCOUNTER
pub struct CounterAttack {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for CounterAttack {
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
        61
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 3 { Ok(3) } else {Err(())}
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
            if 8386559 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
// KN_BOWLINGBASH
pub struct BowlingBash {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for BowlingBash {
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
        62
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 13 { return Ok(13) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
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
       700
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
// KN_RIDING
pub struct PecoPecoRiding {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for PecoPecoRiding {
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
        63
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
// KN_CAVALIERMASTERY
pub struct CavalierMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for CavalierMastery {
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
        64
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
// KN_CHARGEATK
pub struct ChargeAttack {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ChargeAttack {
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
        1001
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
       500
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
// KN_ONEHAND
pub struct OnehandQuicken {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for OnehandQuicken {
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
        495
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 100 { Ok(100) } else {Err(())}
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
            if 4 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
