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
// SM_SWORD
pub struct SwordMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SwordMastery {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_TWOHAND
pub struct TwoHandedSwordMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for TwoHandedSwordMastery {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_RECOVERY
pub struct IncreaseHpRecovery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for IncreaseHpRecovery {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_BASH
pub struct Bash {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Bash {
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
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 8386559 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_PROVOKE
pub struct Provoke {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Provoke {
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
            if character_sp >= 4 { return Ok(4) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 5 { return Ok(5) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 6 { return Ok(6) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 7 { return Ok(7) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 9 { return Ok(9) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 11 { return Ok(11) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 13 { return Ok(13) } else {return Err(())}
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_MAGNUM
pub struct MagnumBreak {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for MagnumBreak {
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
        if character_sp > 30 { Ok(30) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_hp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if character_hp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if character_hp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 4 {
            if character_hp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 5 {
            if character_hp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 6 {
            if character_hp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 7 {
            if character_hp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 8 {
            if character_hp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 9 {
            if character_hp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 10 {
            if character_hp >= 16 { return Ok(16) } else {return Err(())}
        }
        Err(())
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn after_cast_act_delay(&self) -> u32 {
       2000
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_ENDURE
pub struct Endure {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Endure {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_MOVINGRECOVERY
pub struct MovingHpRecovery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for MovingHpRecovery {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_FATALBLOW
pub struct FatalBlow {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FatalBlow {
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// SM_AUTOBERSERK
pub struct AutoBerserk {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for AutoBerserk {
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
        if character_sp > 1 { Ok(1) } else {Err(())}
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn cast_time(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       1
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
