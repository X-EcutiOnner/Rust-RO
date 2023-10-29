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
// ASC_KATAR
pub struct AdvancedKatarMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for AdvancedKatarMastery {
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
    fn hit_count(&self) -> i8 {
       1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// ASC_EDP
pub struct EnchantDeadlyPoison {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for EnchantDeadlyPoison {
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
            if character_sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 5 {
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
    fn hit_count(&self) -> i8 {
       1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
       2000
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// ASC_BREAKER
pub struct SoulDestroyer {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SoulDestroyer {
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
        if self.level == 1 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
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
    fn hit_count(&self) -> i8 {
       1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1400
        }
        if self.level == 4 {
            return 1600
        }
        if self.level == 5 {
            return 1800
        }
        if self.level == 6 {
            return 2000
        }
        if self.level == 7 {
            return 2200
        }
        if self.level == 8 {
            return 2400
        }
        if self.level == 9 {
            return 2600
        }
        if self.level == 10 {
            return 2800
        }
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// ASC_METEORASSAULT
pub struct MeteorAssault {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for MeteorAssault {
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
        if self.level == 1 {
            if character_sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
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
    fn hit_count(&self) -> i8 {
       1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
       500
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// ASC_CDP
pub struct CreateDeadlyPoison {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for CreateDeadlyPoison {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp >= 50 { Ok(50) } else {Err(())}
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
    fn hit_count(&self) -> i8 {
        0
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
       500
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
