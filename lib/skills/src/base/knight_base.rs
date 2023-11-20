// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;

use models::status::Status;
use models::item::NormalInventoryItem;

use crate::{SkillBase, Skill, SkillRequirementResult};

use crate::base::*;
// KN_SPEARMASTERY
pub struct SpearMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SpearMastery {
    fn _id(&self) -> u32 {
        55
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
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
}
// KN_PIERCE
pub struct Pierce {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Pierce {
    fn _id(&self) -> u32 {
        56
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 7 { Ok(7) } else {Err(())}
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       3
    }
}
// KN_BRANDISHSPEAR
pub struct BrandishSpear {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for BrandishSpear {
    fn _id(&self) -> u32 {
        57
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 12 { Ok(12) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Riding
            if status.state & 4 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       700
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// KN_SPEARSTAB
pub struct SpearStab {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SpearStab {
    fn _id(&self) -> u32 {
        58
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 9 { Ok(9) } else {Err(())}
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// KN_SPEARBOOMERANG
pub struct SpearBoomerang {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SpearBoomerang {
    fn _id(&self) -> u32 {
        59
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 10 { Ok(10) } else {Err(())}
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
    }
}
// KN_TWOHANDQUICKEN
pub struct TwohandQuicken {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TwohandQuicken {
    fn _id(&self) -> u32 {
        60
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// KN_AUTOCOUNTER
pub struct CounterAttack {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CounterAttack {
    fn _id(&self) -> u32 {
        61
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 3 { Ok(3) } else {Err(())}
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8386559 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            // Allow to use Fist
            Ok(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// KN_BOWLINGBASH
pub struct BowlingBash {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for BowlingBash {
    fn _id(&self) -> u32 {
        62
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp >= 13 { return Ok(13) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 22 { return Ok(22) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       700
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// KN_RIDING
pub struct PecoPecoRiding {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for PecoPecoRiding {
    fn _id(&self) -> u32 {
        63
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
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
}
// KN_CAVALIERMASTERY
pub struct CavalierMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CavalierMastery {
    fn _id(&self) -> u32 {
        64
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
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
}
// KN_CHARGEATK
pub struct ChargeAttack {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ChargeAttack {
    fn _id(&self) -> u32 {
        1001
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 40 { Ok(40) } else {Err(())}
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       500
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// KN_ONEHAND
pub struct OnehandQuicken {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for OnehandQuicken {
    fn _id(&self) -> u32 {
        495
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
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 100 { Ok(100) } else {Err(())}
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 4 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
