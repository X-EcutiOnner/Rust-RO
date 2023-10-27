// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::weapon::Weapon;
use models::item::NormalInventoryItem;

use crate::{Skill, SkillRequirementResult, DelegateSkill};

use crate::skills::*;
// PR_SLOWPOISON
pub struct SlowPoison {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SlowPoison {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 4 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 6 { return Ok(6) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// HP_ASSUMPTIO
pub struct Assumptio {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Assumptio {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 5 {
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
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// HP_BASILICA
pub struct Basilica {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Basilica {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 110 { return Ok(110) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 120 { return Ok(120) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// HP_MEDITATIO
pub struct Meditatio {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Meditatio {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
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
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// HP_MANARECHARGE
pub struct ManaRecharge {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for ManaRecharge {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
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
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
