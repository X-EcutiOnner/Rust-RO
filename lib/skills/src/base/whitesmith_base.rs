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
// WS_MELTDOWN
pub struct ShatteringStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ShatteringStrike {
    fn _id(&self) -> u32 {
        384
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
            if status.sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 90 { return Ok(90) } else {return Err(())}
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
        if self.level == 1 {
            return 500
        }
        if self.level == 2 {
            return 500
        }
        if self.level == 3 {
            return 600
        }
        if self.level == 4 {
            return 600
        }
        if self.level == 5 {
            return 700
        }
        if self.level == 6 {
            return 700
        }
        if self.level == 7 {
            return 800
        }
        if self.level == 8 {
            return 800
        }
        if self.level == 9 {
            return 900
        }
        if self.level == 10 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// WS_CARTBOOST
pub struct CartBoost {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CartBoost {
    fn _id(&self) -> u32 {
        387
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
        if status.sp > 20 { Ok(20) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Cart
            if status.state & 16 > 0 { Ok(()) } else { Err(()) }
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
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// WS_WEAPONREFINE
pub struct UpgradeWeapon {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for UpgradeWeapon {
    fn _id(&self) -> u32 {
        477
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
        if status.sp > 5 { Ok(5) } else {Err(())}
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
// WS_CARTTERMINATION
pub struct CartTermination {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CartTermination {
    fn _id(&self) -> u32 {
        485
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
        if status.sp > 15 { Ok(15) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // CartBoost
            if status.state & 16777216 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.zeny >= 600 { return Ok(600) } else {return Err(())}
        }
        if self.level == 2 {
            if status.zeny >= 700 { return Ok(700) } else {return Err(())}
        }
        if self.level == 3 {
            if status.zeny >= 800 { return Ok(800) } else {return Err(())}
        }
        if self.level == 4 {
            if status.zeny >= 900 { return Ok(900) } else {return Err(())}
        }
        if self.level == 5 {
            if status.zeny >= 1000 { return Ok(1000) } else {return Err(())}
        }
        if self.level == 6 {
            if status.zeny >= 1100 { return Ok(1100) } else {return Err(())}
        }
        if self.level == 7 {
            if status.zeny >= 1200 { return Ok(1200) } else {return Err(())}
        }
        if self.level == 8 {
            if status.zeny >= 1300 { return Ok(1300) } else {return Err(())}
        }
        if self.level == 9 {
            if status.zeny >= 1400 { return Ok(1400) } else {return Err(())}
        }
        if self.level == 10 {
            if status.zeny >= 1500 { return Ok(1500) } else {return Err(())}
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
// WS_OVERTHRUSTMAX
pub struct MaximumPowerThrust {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MaximumPowerThrust {
    fn _id(&self) -> u32 {
        486
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
        if status.sp > 15 { Ok(15) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.zeny >= 3000 { return Ok(3000) } else {return Err(())}
        }
        if self.level == 2 {
            if status.zeny >= 3500 { return Ok(3500) } else {return Err(())}
        }
        if self.level == 3 {
            if status.zeny >= 4000 { return Ok(4000) } else {return Err(())}
        }
        if self.level == 4 {
            if status.zeny >= 4500 { return Ok(4500) } else {return Err(())}
        }
        if self.level == 5 {
            if status.zeny >= 5000 { return Ok(5000) } else {return Err(())}
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
