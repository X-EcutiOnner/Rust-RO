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
// AS_RIGHT
pub struct RighthandMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for RighthandMastery {
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
    fn cast_delay(&self) -> u32 {
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
// AS_LEFT
pub struct LefthandMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for LefthandMastery {
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
    fn cast_delay(&self) -> u32 {
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
// AS_KATAR
pub struct KatarMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for KatarMastery {
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
    fn cast_delay(&self) -> u32 {
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
// AS_CLOAKING
pub struct Cloaking {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Cloaking {
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
        if character_sp > 15 { Ok(15) } else {Err(())}
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
    fn cast_delay(&self) -> u32 {
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
// AS_SONICBLOW
pub struct SonicBlow {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SonicBlow {
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
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 34 { return Ok(34) } else {return Err(())}
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
        if let Some(character_weapon) = character_weapon {
            if 65536 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn hit_count(&self) -> i8 {
       -8
    }
    fn after_cast_act_delay(&self) -> u32 {
       2000
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// AS_GRIMTOOTH
pub struct Grimtooth {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Grimtooth {
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
        if character_sp > 3 { Ok(3) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            if state.value() == 1 { Ok(()) } else { Err(()) }
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
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 65536 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn cast_delay(&self) -> u32 {
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
// AS_ENCHANTPOISON
pub struct EnchantPoison {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for EnchantPoison {
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
        if character_sp > 20 { Ok(20) } else {Err(())}
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
    fn cast_delay(&self) -> u32 {
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
// AS_POISONREACT
pub struct PoisonReact {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for PoisonReact {
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
            if character_sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 45 { return Ok(45) } else {return Err(())}
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
    fn cast_delay(&self) -> u32 {
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
// AS_VENOMDUST
pub struct VenomDust {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for VenomDust {
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
        if character_sp > 20 { Ok(20) } else {Err(())}
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
    fn cast_delay(&self) -> u32 {
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
// AS_SPLASHER
pub struct VenomSplasher {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for VenomSplasher {
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
    fn cast_delay(&self) -> u32 {
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
// AS_SONICACCEL
pub struct SonicAcceleration {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SonicAcceleration {
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
    fn cast_delay(&self) -> u32 {
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
// AS_VENOMKNIFE
pub struct ThrowVenomKnife {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for ThrowVenomKnife {
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
        if character_sp > 15 { Ok(15) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        if let Some(ammo_and_amount) = character_ammo {
            if ammo_and_amount.1 >= 1 && (4 & ammo_and_amount.0.as_flag()) > 0 { Ok(1) } else { Err(()) }
        } else {
            Err(())
        }
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
    fn cast_delay(&self) -> u32 {
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
