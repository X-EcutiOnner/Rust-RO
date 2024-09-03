// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use models::enums::skill::*;
use models::enums::weapon::AmmoType;
use models::enums::element::Element::{*};

use models::item::WearWeapon;

use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;
use models::enums::weapon::WeaponType::{*};
use models::enums::bonus::{BonusType};
use models::enums::status::StatusEffect::{*};
use models::status_bonus::{TemporaryStatusBonus};
use models::enums::mob::MobRace::{*};

use crate::{*};

use crate::base::*;
use std::any::Any;
// CG_ARROWVULCAN - Vulcan Arrow
pub struct VulcanArrow {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for VulcanArrow {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        394
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
       9
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
            return 14
        }
        if self.level == 3 {
            return 16
        }
        if self.level == 4 {
            return 18
        }
        if self.level == 5 {
            return 20
        }
        if self.level == 6 {
            return 22
        }
        if self.level == 7 {
            return 24
        }
        if self.level == 8 {
            return 26
        }
        if self.level == 9 {
            return 28
        }
        if self.level == 10 {
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
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        if let Some(ammo_and_amount) = character_ammo {
            if ammo_and_amount.1 >= 1 && (2 & ammo_and_amount.0.as_flag()) > 0 { Ok(1) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 24576 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 2200
        }
        if self.level == 3 {
            return 2400
        }
        if self.level == 4 {
            return 2600
        }
        if self.level == 5 {
            return 2800
        }
        if self.level == 6 {
            return 3000
        }
        if self.level == 7 {
            return 3200
        }
        if self.level == 8 {
            return 3400
        }
        if self.level == 9 {
            return 3600
        }
        if self.level == 10 {
            return 3800
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 2800
        }
        if self.level == 2 {
            return 2800
        }
        if self.level == 3 {
            return 2800
        }
        if self.level == 4 {
            return 2800
        }
        if self.level == 5 {
            return 2800
        }
        if self.level == 6 {
            return 3000
        }
        if self.level == 7 {
            return 3000
        }
        if self.level == 8 {
            return 3000
        }
        if self.level == 9 {
            return 3000
        }
        if self.level == 10 {
            return 3000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 2800
        }
        if self.level == 2 {
            return 2800
        }
        if self.level == 3 {
            return 2800
        }
        if self.level == 4 {
            return 2800
        }
        if self.level == 5 {
            return 2800
        }
        if self.level == 6 {
            return 3000
        }
        if self.level == 7 {
            return 3000
        }
        if self.level == 8 {
            return 3000
        }
        if self.level == 9 {
            return 3000
        }
        if self.level == 10 {
            return 3000
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
impl OffensiveSkillBase for VulcanArrow {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       -9
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(3.000)
        }
        if self.level == 2 {
            return Some(4.000)
        }
        if self.level == 3 {
            return Some(5.000)
        }
        if self.level == 4 {
            return Some(6.000)
        }
        if self.level == 5 {
            return Some(7.000)
        }
        if self.level == 6 {
            return Some(8.000)
        }
        if self.level == 7 {
            return Some(9.000)
        }
        if self.level == 8 {
            return Some(10.000)
        }
        if self.level == 9 {
            return Some(11.000)
        }
        if self.level == 10 {
            return Some(12.000)
        }
        None
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Ammo
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
// CG_MOONLIT - Sheltering Bliss
pub struct ShelteringBliss {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ShelteringBliss {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        395
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
            return 30
        }
        if self.level == 2 {
            return 40
        }
        if self.level == 3 {
            return 50
        }
        if self.level == 4 {
            return 60
        }
        if self.level == 5 {
            return 70
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
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 24576 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn is_performance_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_performance_skill(&self) -> Option<&dyn PerformanceSkill> {
        Some(self)
    }
}
impl PerformanceSkillBase for ShelteringBliss {
}
// CG_MARIONETTE - Marionette Control
pub struct MarionetteControl {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MarionetteControl {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        396
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
       7
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       100
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 100 { Ok(100) } else {Err(())}
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
        16
    }
}
impl SupportiveSkillBase for MarionetteControl {
}
// CG_LONGINGFREEDOM - Longing for Freedom
pub struct LongingforFreedom {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for LongingforFreedom {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        487
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
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-40.0), 2, tick, 180000),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-40), 2, tick, 180000),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-30.0), 2, tick, 180000),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-30), 2, tick, 180000),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-20.0), 2, tick, 180000),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-20), 2, tick, 180000),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-10.0), 2, tick, 180000),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-10), 2, tick, 180000),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(0.0), 2, tick, 180000),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(0), 2, tick, 180000),]);
        }
        TemporaryStatusBonuses::default()
    }
}
// CG_HERMODE - Wand of Hermode
pub struct WandofHermode {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for WandofHermode {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        488
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
            return 20
        }
        if self.level == 2 {
            return 30
        }
        if self.level == 3 {
            return 40
        }
        if self.level == 4 {
            return 50
        }
        if self.level == 5 {
            return 60
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
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 24576 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn is_performance_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_performance_skill(&self) -> Option<&dyn PerformanceSkill> {
        Some(self)
    }
}
impl PerformanceSkillBase for WandofHermode {
}
// CG_TAROTCARD - Tarot Card of Fate
pub struct TarotCardofFate {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TarotCardofFate {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        489
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
       9
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
       40
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }
    fn _is_magic(&self) -> bool {
        true
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 40 { Ok(40) } else {Err(())}
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       3000
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
impl OffensiveSkillBase for TarotCardofFate {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Neutral
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
