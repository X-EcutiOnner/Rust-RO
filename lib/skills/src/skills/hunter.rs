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
// HT_SKIDTRAP
pub struct SkidTrap {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SkidTrap {
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
        if character_sp >= 10 { Ok(10) } else {Err(())}
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
// HT_LANDMINE
pub struct LandMine {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for LandMine {
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
        if character_sp >= 10 { Ok(10) } else {Err(())}
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
// HT_ANKLESNARE
pub struct AnkleSnare {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for AnkleSnare {
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
        if character_sp >= 12 { Ok(12) } else {Err(())}
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
// HT_SHOCKWAVE
pub struct ShockwaveTrap {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for ShockwaveTrap {
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
        if character_sp >= 45 { Ok(45) } else {Err(())}
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
// HT_SANDMAN
pub struct Sandman {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Sandman {
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
        if character_sp >= 12 { Ok(12) } else {Err(())}
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
// HT_FLASHER
pub struct Flasher {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Flasher {
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
        if character_sp >= 12 { Ok(12) } else {Err(())}
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
// HT_FREEZINGTRAP
pub struct FreezingTrap {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FreezingTrap {
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
        if character_sp >= 10 { Ok(10) } else {Err(())}
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
// HT_BLASTMINE
pub struct BlastMine {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for BlastMine {
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
        if character_sp >= 10 { Ok(10) } else {Err(())}
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
// HT_CLAYMORETRAP
pub struct ClaymoreTrap {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for ClaymoreTrap {
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
        if character_sp >= 15 { Ok(15) } else {Err(())}
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
// HT_REMOVETRAP
pub struct RemoveTrap {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for RemoveTrap {
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
        if character_sp >= 5 { Ok(5) } else {Err(())}
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
// HT_TALKIEBOX
pub struct TalkieBox {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for TalkieBox {
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
        if character_sp >= 1 { Ok(1) } else {Err(())}
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
// HT_BEASTBANE
pub struct BeastBane {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for BeastBane {
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
// HT_FALCON
pub struct FalconryMastery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FalconryMastery {
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
// HT_STEELCROW
pub struct SteelCrow {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SteelCrow {
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
// HT_BLITZBEAT
pub struct BlitzBeat {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for BlitzBeat {
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
            if character_sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 13 { return Ok(13) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 5 {
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
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            if state.value() == 3 { Ok(()) } else { Err(()) }
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
// HT_DETECTING
pub struct Detect {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Detect {
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
        if character_sp >= 8 { Ok(8) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            if state.value() == 3 { Ok(()) } else { Err(()) }
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
// HT_SPRINGTRAP
pub struct SpringTrap {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SpringTrap {
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
        if character_sp >= 10 { Ok(10) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        if let Some(state) = state {
            if state.value() == 3 { Ok(()) } else { Err(()) }
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
// HT_PHANTASMIC
pub struct PhantasmicArrow {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for PhantasmicArrow {
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
        if character_sp >= 10 { Ok(10) } else {Err(())}
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
            if 2048 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
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
// HT_POWER
pub struct BeastStrafing {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for BeastStrafing {
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
        if character_sp >= 12 { Ok(12) } else {Err(())}
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
            if 2048 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
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
