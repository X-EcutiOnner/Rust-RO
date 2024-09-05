// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::weapon::AmmoType;
use models::enums::element::Element::{*};

use models::item::WearWeapon;

use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;
use models::enums::weapon::WeaponType::{*};
use models::enums::bonus::{BonusType};
use models::enums::status::StatusEffect::{*};
use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonus};
use models::enums::mob::MobRace::{*};

use crate::{*};

use crate::base::*;
use std::any::Any;
// LK_AURABLADE - Aura Blade
pub struct AuraBlade {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AuraBlade {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        355
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 18
        }
        if self.level == 2 {
            return 26
        }
        if self.level == 3 {
            return 34
        }
        if self.level == 4 {
            return 42
        }
        if self.level == 5 {
            return 50
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8388606 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(20), 14, tick, 40000, 355),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(40), 14, tick, 60000, 355),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(60), 14, tick, 80000, 355),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(80), 14, tick, 100000, 355),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(100), 14, tick, 120000, 355),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        4
    }
}
impl SupportiveSkillBase for AuraBlade {
}
// LK_PARRYING - Parrying
pub struct Parrying {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Parrying {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        356
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        10
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       50
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 50 { Ok(50) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(5), 14, tick, 15000, 356),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(10), 14, tick, 20000, 356),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(15), 14, tick, 25000, 356),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(20), 14, tick, 30000, 356),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(25), 14, tick, 35000, 356),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(30), 14, tick, 40000, 356),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(35), 14, tick, 45000, 356),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(40), 14, tick, 50000, 356),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(45), 14, tick, 55000, 356),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(50), 14, tick, 60000, 356),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        4
    }
}
impl SupportiveSkillBase for Parrying {
}
// LK_CONCENTRATION - Concentration
pub struct Concentration {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Concentration {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        357
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 14
        }
        if self.level == 2 {
            return 18
        }
        if self.level == 3 {
            return 22
        }
        if self.level == 4 {
            return 26
        }
        if self.level == 5 {
            return 30
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(5), 14, tick, 25000, 357),
                TemporaryStatusBonus::with_duration(BonusType::Def(-5), 14, tick, 25000, 357),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(10), 14, tick, 30000, 357),
                TemporaryStatusBonus::with_duration(BonusType::Def(-10), 14, tick, 30000, 357),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(15), 14, tick, 35000, 357),
                TemporaryStatusBonus::with_duration(BonusType::Def(-15), 14, tick, 35000, 357),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(20), 14, tick, 40000, 357),
                TemporaryStatusBonus::with_duration(BonusType::Def(-20), 14, tick, 40000, 357),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(25), 14, tick, 45000, 357),
                TemporaryStatusBonus::with_duration(BonusType::Def(-25), 14, tick, 45000, 357),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        4
    }
}
impl SupportiveSkillBase for Concentration {
}
// LK_TENSIONRELAX - Relax
pub struct Relax {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Relax {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        358
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       15
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 15 { Ok(15) } else {Err(())}
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        4
    }
}
impl SupportiveSkillBase for Relax {
}
// LK_BERSERK - Frenzy
pub struct Frenzy {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Frenzy {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        359
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       200
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 200 { Ok(200) } else {Err(())}
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
    #[inline(always)]
    fn _client_type(&self) -> usize {
        4
    }
}
impl SupportiveSkillBase for Frenzy {
}
// LK_SPIRALPIERCE - Spiral Pierce
pub struct SpiralPierce {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SpiralPierce {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        397
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       5
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 18
        }
        if self.level == 2 {
            return 21
        }
        if self.level == 3 {
            return 24
        }
        if self.level == 4 {
            return 27
        }
        if self.level == 5 {
            return 30
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 300
        }
        if self.level == 2 {
            return 500
        }
        if self.level == 3 {
            return 700
        }
        if self.level == 4 {
            return 900
        }
        if self.level == 5 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 1200
        }
        if self.level == 2 {
            return 1400
        }
        if self.level == 3 {
            return 1600
        }
        if self.level == 4 {
            return 1800
        }
        if self.level == 5 {
            return 2000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 1200
        }
        if self.level == 2 {
            return 1400
        }
        if self.level == 3 {
            return 1600
        }
        if self.level == 4 {
            return 1800
        }
        if self.level == 5 {
            return 2000
        }
        0
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for SpiralPierce {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       5
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
// LK_HEADCRUSH - Traumatic Blow
pub struct TraumaticBlow {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TraumaticBlow {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        398
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       4
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       23
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 23 { Ok(23) } else {Err(())}
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       500
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for TraumaticBlow {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.400)
        }
        if self.level == 2 {
            return Some(1.800)
        }
        if self.level == 3 {
            return Some(2.200)
        }
        if self.level == 4 {
            return Some(2.600)
        }
        if self.level == 5 {
            return Some(3.000)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        let mut effects = Vec::with_capacity(1);
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 50 {
                effects.push(StatusEffect::Bleeding);
            }
        }
        if self.level == 2 {
            if chance <= 50 {
                effects.push(StatusEffect::Bleeding);
            }
        }
        if self.level == 3 {
            if chance <= 50 {
                effects.push(StatusEffect::Bleeding);
            }
        }
        if self.level == 4 {
            if chance <= 50 {
                effects.push(StatusEffect::Bleeding);
            }
        }
        if self.level == 5 {
            if chance <= 50 {
                effects.push(StatusEffect::Bleeding);
            }
        }
        effects
    }
}
// LK_JOINTBEAT - Vital Strike
pub struct VitalStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for VitalStrike {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        399
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       4
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        10
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 12
        }
        if self.level == 2 {
            return 12
        }
        if self.level == 3 {
            return 14
        }
        if self.level == 4 {
            return 14
        }
        if self.level == 5 {
            return 16
        }
        if self.level == 6 {
            return 16
        }
        if self.level == 7 {
            return 18
        }
        if self.level == 8 {
            return 18
        }
        if self.level == 9 {
            return 20
        }
        if self.level == 10 {
            return 20
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 800
        }
        if self.level == 2 {
            return 800
        }
        if self.level == 3 {
            return 800
        }
        if self.level == 4 {
            return 800
        }
        if self.level == 5 {
            return 800
        }
        if self.level == 6 {
            return 1000
        }
        if self.level == 7 {
            return 1000
        }
        if self.level == 8 {
            return 1000
        }
        if self.level == 9 {
            return 1000
        }
        if self.level == 10 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 800
        }
        if self.level == 2 {
            return 800
        }
        if self.level == 3 {
            return 800
        }
        if self.level == 4 {
            return 800
        }
        if self.level == 5 {
            return 800
        }
        if self.level == 6 {
            return 1000
        }
        if self.level == 7 {
            return 1000
        }
        if self.level == 8 {
            return 1000
        }
        if self.level == 9 {
            return 1000
        }
        if self.level == 10 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for VitalStrike {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(0.600)
        }
        if self.level == 2 {
            return Some(0.700)
        }
        if self.level == 3 {
            return Some(0.800)
        }
        if self.level == 4 {
            return Some(0.900)
        }
        if self.level == 5 {
            return Some(1.000)
        }
        if self.level == 6 {
            return Some(1.100)
        }
        if self.level == 7 {
            return Some(1.200)
        }
        if self.level == 8 {
            return Some(1.300)
        }
        if self.level == 9 {
            return Some(1.400)
        }
        if self.level == 10 {
            return Some(1.500)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Weapon
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        let mut effects = Vec::with_capacity(1);
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 5 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 2 {
            if chance <= 15 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 3 {
            if chance <= 20 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 4 {
            if chance <= 25 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 5 {
            if chance <= 30 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 6 {
            if chance <= 35 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 7 {
            if chance <= 40 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 8 {
            if chance <= 45 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 9 {
            if chance <= 50 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        if self.level == 10 {
            if chance <= 55 {
                effects.push(StatusEffect::VitalStrike);
            }
        }
        effects
    }
}
