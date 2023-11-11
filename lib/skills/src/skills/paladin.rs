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
// PA_PRESSURE
pub struct GloriaDomini {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for GloriaDomini {
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
        367
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 5 {
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
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 2500
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 3500
        }
        if self.level == 5 {
            return 4000
        }
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 2500
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 3500
        }
        if self.level == 5 {
            return 4000
        }
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 2500
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 3500
        }
        if self.level == 5 {
            return 4000
        }
        0
    }
}
// PA_SACRIFICE
pub struct MartyrsReckoning {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for MartyrsReckoning {
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
        368
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
       2000
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// PA_GOSPEL
pub struct BattleChant {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for BattleChant {
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
        369
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 100 { return Ok(100) } else {return Err(())}
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
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// PA_SHIELDCHAIN
pub struct ShieldChain {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
    cast_time: u32,
    after_cast_act_delay: u32,
    after_cast_walk_delay: u32,
}
impl Skill for ShieldChain {
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
        480
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
       5
    }
    fn base_after_cast_act_delay(&self) -> u32 {
       1000
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
