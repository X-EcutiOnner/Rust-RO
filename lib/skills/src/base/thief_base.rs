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
// TF_DOUBLE - Double Attack
pub struct DoubleAttack {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for DoubleAttack {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        48
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
       -1
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
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Passive
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(1), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 5.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(2), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 10.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(3), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 15.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(4), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 20.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(5), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 25.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(6), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 30.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(7), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 35.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(8), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 40.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(9), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 45.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::AccuracyPercentage(10), StatusBonusFlag::Default.as_flag(), 48),
                TemporaryStatusBonus::with_passive_skill(BonusType::AutospellSkillIdChancePercentage(48, 50.0), StatusBonusFlag::Default.as_flag(), 48),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_passive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {
        Some(self)
    }
}
impl PassiveSkillBase for DoubleAttack {
}
// TF_MISS - Improve Dodge
pub struct ImproveDodge {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ImproveDodge {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        49
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
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Passive
    }
    fn _is_magic(&self) -> bool {
        false
    }
    fn _is_physical(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(1), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(2), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(3), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(4), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(5), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(6), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(7), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(8), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(9), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpeedPercentage(10), StatusBonusFlag::Default.as_flag(), 49),]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_passive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {
        Some(self)
    }
}
impl PassiveSkillBase for ImproveDodge {
}
// TF_STEAL - Steal
pub struct Steal {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Steal {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        50
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
       1
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
       10
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
        if status.sp() > 10 { Ok(10) } else {Err(())}
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
        if self.level == 6 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses(vec![]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses(vec![]);
        }
        TemporaryStatusBonuses::default()
    }
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_interactive_skill(&self) -> Option<&dyn InteractiveSkill> {
        Some(self)
    }
}
impl InteractiveSkillBase for Steal {
}
// TF_HIDING - Hiding
pub struct Hiding {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Hiding {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        51
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
       1
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
       10
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
        if status.sp() > 10 { Ok(10) } else {Err(())}
    }
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_interactive_skill(&self) -> Option<&dyn InteractiveSkill> {
        Some(self)
    }
}
impl InteractiveSkillBase for Hiding {
}
// TF_POISON - Envenom
pub struct Envenom {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Envenom {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        52
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
       -2
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
       12
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
        if status.sp() > 12 { Ok(12) } else {Err(())}
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
impl OffensiveSkillBase for Envenom {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
       Some(1.000)
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Poison
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        let mut effects = Vec::with_capacity(1);
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 14 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 2 {
            if chance <= 18 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 3 {
            if chance <= 22 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 4 {
            if chance <= 26 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 5 {
            if chance <= 30 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 6 {
            if chance <= 34 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 7 {
            if chance <= 38 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 8 {
            if chance <= 42 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 9 {
            if chance <= 46 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        if self.level == 10 {
            if chance <= 50 {
                effects.push(StatusEffect::Poisoned);
            }
        }
        effects
    }
}
// TF_DETOXIFY - Detoxify
pub struct Detoxify {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Detoxify {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        53
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
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       10
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
        if status.sp() > 10 { Ok(10) } else {Err(())}
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
impl SupportiveSkillBase for Detoxify {
}
// TF_SPRINKLESAND - Sand Attack
pub struct SandAttack {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SandAttack {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        149
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
       1
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
       9
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
        if status.sp() > 9 { Ok(9) } else {Err(())}
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
impl OffensiveSkillBase for SandAttack {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
       Some(1.300)
    }
    #[inline(always)]
    fn _element(&self) -> Element {
        Element::Earth
    }
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        let mut effects = Vec::with_capacity(1);
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 20 {
                effects.push(StatusEffect::Blind);
            }
        }
        effects
    }
}
// TF_BACKSLIDING - Back Slide
pub struct BackSlide {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for BackSlide {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        150
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
       7
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
        if status.sp() > 7 { Ok(7) } else {Err(())}
    }
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_interactive_skill(&self) -> Option<&dyn InteractiveSkill> {
        Some(self)
    }
}
impl InteractiveSkillBase for BackSlide {
}
// TF_PICKSTONE - Find Stone
pub struct FindStone {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FindStone {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        151
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
       3
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
        if status.sp() > 3 { Ok(3) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // RecoverWeightRate
            if status.state() & 64 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       500
    }
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_interactive_skill(&self) -> Option<&dyn InteractiveSkill> {
        Some(self)
    }
}
impl InteractiveSkillBase for FindStone {
}
// TF_THROWSTONE - Stone Fling
pub struct StoneFling {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for StoneFling {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        152
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
       2
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
        if status.sp() > 2 { Ok(2) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 7049, name_english: "Stone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 7049 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
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
impl OffensiveSkillBase for StoneFling {
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
        let mut effects = Vec::with_capacity(2);
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 3 {
                effects.push(StatusEffect::Stun);
            }
        }
        let chance = _rng.u8(1..=100);
        if self.level == 1 {
            if chance <= 3 {
                effects.push(StatusEffect::Blind);
            }
        }
        effects
    }
}
