// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;

use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;

use crate::{*};

use crate::base::*;
use std::any::Any;
// SN_SIGHT
pub struct FalconEyes {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FalconEyes {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        380
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
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if *status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 7 {
            if *status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 8 {
            if *status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 9 {
            if *status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 10 {
            if *status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for FalconEyes {
}
// SN_FALCONASSAULT
pub struct FalconAssault {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FalconAssault {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        381
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
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if *status.state() > 0 {
            // Falcon
            if *status.state() & 8 > 0 { Ok(()) } else { Err(()) }
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
impl OffensiveSkillBase for FalconAssault {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// SN_SHARPSHOOTING
pub struct FocusedArrowStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FocusedArrowStrike {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        382
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
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 30 { return Ok(30) } else {return Err(())}
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
            if 2048 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1500
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
impl OffensiveSkillBase for FocusedArrowStrike {
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
            return Some(2.800)
        }
        if self.level == 3 {
            return Some(3.200)
        }
        if self.level == 4 {
            return Some(4.600)
        }
        if self.level == 5 {
            return Some(6.000)
        }
        None
    }
}
// SN_WINDWALK
pub struct WindWalker {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for WindWalker {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        383
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
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 58 { return Ok(58) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 6 {
            if *status.sp() >= 76 { return Ok(76) } else {return Err(())}
        }
        if self.level == 7 {
            if *status.sp() >= 82 { return Ok(82) } else {return Err(())}
        }
        if self.level == 8 {
            if *status.sp() >= 88 { return Ok(88) } else {return Err(())}
        }
        if self.level == 9 {
            if *status.sp() >= 94 { return Ok(94) } else {return Err(())}
        }
        if self.level == 10 {
            if *status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 2400
        }
        if self.level == 3 {
            return 2800
        }
        if self.level == 4 {
            return 3200
        }
        if self.level == 5 {
            return 3600
        }
        if self.level == 6 {
            return 4000
        }
        if self.level == 7 {
            return 4400
        }
        if self.level == 8 {
            return 4800
        }
        if self.level == 9 {
            return 5200
        }
        if self.level == 10 {
            return 5600
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for WindWalker {
}
