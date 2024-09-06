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
// PA_PRESSURE - Gloria Domini
pub struct GloriaDomini {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for GloriaDomini {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        367
    }
    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
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
        if self.level == 1 {
            return 30
        }
        if self.level == 2 {
            return 35
        }
        if self.level == 3 {
            return 40
        }
        if self.level == 4 {
            return 45
        }
        if self.level == 5 {
            return 50
        }
        0
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
        if self.level == 1 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
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
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
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
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
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
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for GloriaDomini {
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
// PA_SACRIFICE - Martyr's Reckoning
pub struct MartyrsReckoning {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MartyrsReckoning {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        368
    }
    fn skill_type(&self) -> SkillType {
        SkillType::Support
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
       100
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
        if status.sp() > 100 { Ok(100) } else {Err(())}
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
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
impl SupportiveSkillBase for MartyrsReckoning {
}
// PA_GOSPEL - Battle Chant
pub struct BattleChant {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for BattleChant {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        369
    }
    fn skill_type(&self) -> SkillType {
        SkillType::Performance
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
        if self.level == 1 {
            return 80
        }
        if self.level == 2 {
            return 80
        }
        if self.level == 3 {
            return 80
        }
        if self.level == 4 {
            return 80
        }
        if self.level == 5 {
            return 80
        }
        if self.level == 6 {
            return 100
        }
        if self.level == 7 {
            return 100
        }
        if self.level == 8 {
            return 100
        }
        if self.level == 9 {
            return 100
        }
        if self.level == 10 {
            return 100
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
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
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
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 55.0), 0, tick, 60000, 369),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 60.0), 0, tick, 60000, 369),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 65.0), 0, tick, 60000, 369),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 70.0), 0, tick, 60000, 369),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 75.0), 0, tick, 60000, 369),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 80.0), 0, tick, 60000, 369),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 85.0), 0, tick, 60000, 369),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 90.0), 0, tick, 60000, 369),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 95.0), 0, tick, 60000, 369),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(369, 100.0), 0, tick, 60000, 369),]);
        }
        TemporaryStatusBonuses::default()
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
impl PerformanceSkillBase for BattleChant {
}
// PA_SHIELDCHAIN - Shield Chain
pub struct ShieldChain {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ShieldChain {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        480
    }
    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
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
        if self.level == 1 {
            return 28
        }
        if self.level == 2 {
            return 31
        }
        if self.level == 3 {
            return 34
        }
        if self.level == 4 {
            return 37
        }
        if self.level == 5 {
            return 40
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
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 31 { return Ok(31) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // Shield
            if status.state() & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
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
impl OffensiveSkillBase for ShieldChain {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       5
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(6.500)
        }
        if self.level == 2 {
            return Some(8.000)
        }
        if self.level == 3 {
            return Some(9.500)
        }
        if self.level == 4 {
            return Some(11.000)
        }
        if self.level == 5 {
            return Some(12.500)
        }
        None
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
