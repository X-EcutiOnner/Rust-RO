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
// MC_INCCARRY
pub struct EnlargeWeightLimit {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for EnlargeWeightLimit {
    fn _id(&self) -> u32 {
        36
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
        0
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_DISCOUNT
pub struct Discount {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Discount {
    fn _id(&self) -> u32 {
        37
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
        0
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_OVERCHARGE
pub struct Overcharge {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Overcharge {
    fn _id(&self) -> u32 {
        38
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
        0
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_PUSHCART
pub struct Pushcart {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Pushcart {
    fn _id(&self) -> u32 {
        39
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
        0
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_IDENTIFY
pub struct ItemAppraisal {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ItemAppraisal {
    fn _id(&self) -> u32 {
        40
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 10 { Ok(10) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_VENDING
pub struct Vending {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Vending {
    fn _id(&self) -> u32 {
        41
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 30 { Ok(30) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Cart
            if status.state & 16 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_MAMMONITE
pub struct Mammonite {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Mammonite {
    fn _id(&self) -> u32 {
        42
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 5 { Ok(5) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.zeny >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 2 {
            if status.zeny >= 200 { return Ok(200) } else {return Err(())}
        }
        if self.level == 3 {
            if status.zeny >= 300 { return Ok(300) } else {return Err(())}
        }
        if self.level == 4 {
            if status.zeny >= 400 { return Ok(400) } else {return Err(())}
        }
        if self.level == 5 {
            if status.zeny >= 500 { return Ok(500) } else {return Err(())}
        }
        if self.level == 6 {
            if status.zeny >= 600 { return Ok(600) } else {return Err(())}
        }
        if self.level == 7 {
            if status.zeny >= 700 { return Ok(700) } else {return Err(())}
        }
        if self.level == 8 {
            if status.zeny >= 800 { return Ok(800) } else {return Err(())}
        }
        if self.level == 9 {
            if status.zeny >= 900 { return Ok(900) } else {return Err(())}
        }
        if self.level == 10 {
            if status.zeny >= 1000 { return Ok(1000) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_CARTREVOLUTION
pub struct CartRevolution {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CartRevolution {
    fn _id(&self) -> u32 {
        153
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 12 { Ok(12) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Cart
            if status.state & 16 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_CHANGECART
pub struct ChangeCart {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for ChangeCart {
    fn _id(&self) -> u32 {
        154
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 40 { Ok(40) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Cart
            if status.state & 16 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_LOUD
pub struct CrazyUproar {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for CrazyUproar {
    fn _id(&self) -> u32 {
        155
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 8 { Ok(8) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// MC_CARTDECORATE
pub struct DecorateCart {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for DecorateCart {
    fn _id(&self) -> u32 {
        2544
    }
    fn _level(&self) -> u8 {
        self.level
    }
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 40 { Ok(40) } else {Err(())}
    }
    fn _validate_hp(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Cart
            if status.state & 16 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_zeny(&self, status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self,status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, status: &Status) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _hit_count(&self) -> i8 {
       1
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
