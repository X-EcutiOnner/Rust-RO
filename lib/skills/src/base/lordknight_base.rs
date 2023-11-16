// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;
use models::item::NormalInventoryItem;

use crate::{SkillBase, Skill, SkillRequirementResult};

use crate::base::*;
// LK_AURABLADE
pub struct AuraBlade {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AuraBlade {
    fn _id(&self) -> u32 {
        355
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 8388606 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
// LK_PARRYING
pub struct Parrying {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Parrying {
    fn _id(&self) -> u32 {
        356
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 50 { Ok(50) } else {Err(())}
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 8 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
// LK_CONCENTRATION
pub struct Concentration {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Concentration {
    fn _id(&self) -> u32 {
        357
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
// LK_TENSIONRELAX
pub struct Relax {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Relax {
    fn _id(&self) -> u32 {
        358
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 15 { Ok(15) } else {Err(())}
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
// LK_BERSERK
pub struct Frenzy {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Frenzy {
    fn _id(&self) -> u32 {
        359
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 200 { Ok(200) } else {Err(())}
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
// LK_SPIRALPIERCE
pub struct SpiralPierce {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SpiralPierce {
    fn _id(&self) -> u32 {
        397
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn _skip_item_validation(&self, state: Option<u64>) -> bool {
        false
    }
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
    fn _hit_count(&self) -> i8 {
       5
    }
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
}
// LK_HEADCRUSH
pub struct TraumaticBlow {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TraumaticBlow {
    fn _id(&self) -> u32 {
        398
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp > 23 { Ok(23) } else {Err(())}
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
       500
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }
}
// LK_JOINTBEAT
pub struct VitalStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for VitalStrike {
    fn _id(&self) -> u32 {
        399
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
    fn _validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        Err(())
    }
    fn _validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    fn _validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = character_weapon {
            if 48 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    fn _validate_range(&self, character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
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
}
