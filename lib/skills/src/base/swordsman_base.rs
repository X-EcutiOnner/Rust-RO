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
// SM_SWORD
pub struct SwordMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SwordMastery {
    fn _id(&self) -> u32 {
        2
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
// SM_TWOHAND
pub struct TwoHandedSwordMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TwoHandedSwordMastery {
    fn _id(&self) -> u32 {
        3
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
// SM_RECOVERY
pub struct IncreaseHpRecovery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for IncreaseHpRecovery {
    fn _id(&self) -> u32 {
        4
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
// SM_BASH
pub struct Bash {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Bash {
    fn _id(&self) -> u32 {
        5
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
            if status.sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
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
// SM_PROVOKE
pub struct Provoke {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Provoke {
    fn _id(&self) -> u32 {
        6
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
            if status.sp >= 4 { return Ok(4) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 5 { return Ok(5) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 6 { return Ok(6) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 7 { return Ok(7) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 9 { return Ok(9) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 11 { return Ok(11) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 13 { return Ok(13) } else {return Err(())}
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
    fn _hit_count(&self) -> i8 {
       1
    }
}
// SM_MAGNUM
pub struct MagnumBreak {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MagnumBreak {
    fn _id(&self) -> u32 {
        7
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
        if status.sp > 30 { Ok(30) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.hp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.hp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.hp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 4 {
            if status.hp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 5 {
            if status.hp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 6 {
            if status.hp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 7 {
            if status.hp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 8 {
            if status.hp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 9 {
            if status.hp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 10 {
            if status.hp >= 16 { return Ok(16) } else {return Err(())}
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
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
}
// SM_ENDURE
pub struct Endure {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Endure {
    fn _id(&self) -> u32 {
        8
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
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// SM_MOVINGRECOVERY
pub struct MovingHpRecovery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MovingHpRecovery {
    fn _id(&self) -> u32 {
        144
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
// SM_FATALBLOW
pub struct FatalBlow {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FatalBlow {
    fn _id(&self) -> u32 {
        145
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
// SM_AUTOBERSERK
pub struct AutoBerserk {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AutoBerserk {
    fn _id(&self) -> u32 {
        146
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
        if status.sp > 1 { Ok(1) } else {Err(())}
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
    fn _hit_count(&self) -> i8 {
       1
    }
}
