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
// MC_INCCARRY
pub struct EnlargeWeightLimit {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for EnlargeWeightLimit {
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
        36
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
// MC_DISCOUNT
pub struct Discount {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Discount {
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
        37
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
// MC_OVERCHARGE
pub struct Overcharge {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Overcharge {
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
        38
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
// MC_PUSHCART
pub struct Pushcart {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Pushcart {
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
        39
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
// MC_IDENTIFY
pub struct ItemAppraisal {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ItemAppraisal {
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
        40
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
// MC_VENDING
pub struct Vending {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Vending {
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
        41
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
            // Cart
            if state & 16 > 0 { Ok(()) } else { Err(()) }
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
// MC_MAMMONITE
pub struct Mammonite {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for Mammonite {
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
        42
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 5 { Ok(5) } else {Err(())}
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
        if self.level == 1 {
            if zeny >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 2 {
            if zeny >= 200 { return Ok(200) } else {return Err(())}
        }
        if self.level == 3 {
            if zeny >= 300 { return Ok(300) } else {return Err(())}
        }
        if self.level == 4 {
            if zeny >= 400 { return Ok(400) } else {return Err(())}
        }
        if self.level == 5 {
            if zeny >= 500 { return Ok(500) } else {return Err(())}
        }
        if self.level == 6 {
            if zeny >= 600 { return Ok(600) } else {return Err(())}
        }
        if self.level == 7 {
            if zeny >= 700 { return Ok(700) } else {return Err(())}
        }
        if self.level == 8 {
            if zeny >= 800 { return Ok(800) } else {return Err(())}
        }
        if self.level == 9 {
            if zeny >= 900 { return Ok(900) } else {return Err(())}
        }
        if self.level == 10 {
            if zeny >= 1000 { return Ok(1000) } else {return Err(())}
        }
        Err(())
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
// MC_CARTREVOLUTION
pub struct CartRevolution {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for CartRevolution {
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
        153
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
            // Cart
            if state & 16 > 0 { Ok(()) } else { Err(()) }
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
// MC_CHANGECART
pub struct ChangeCart {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ChangeCart {
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
        154
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
        if let Some(state) = state {
            // Cart
            if state & 16 > 0 { Ok(()) } else { Err(()) }
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
// MC_LOUD
pub struct CrazyUproar {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for CrazyUproar {
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
        155
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 8 { Ok(8) } else {Err(())}
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
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_CARTDECORATE
pub struct DecorateCart {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for DecorateCart {
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
        2544
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
        if let Some(state) = state {
            // Cart
            if state & 16 > 0 { Ok(()) } else { Err(()) }
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
