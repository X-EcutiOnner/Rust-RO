// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;

use models::status::Status;
use models::item::NormalInventoryItem;

use crate::{*};

use crate::base::*;
use std::any::Any;
// WZ_FIREPILLAR
pub struct FirePillar {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FirePillar {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        80
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
        if status.sp > 75 { Ok(75) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 3000
        }
        if self.level == 2 {
            return 2700
        }
        if self.level == 3 {
            return 2400
        }
        if self.level == 4 {
            return 2100
        }
        if self.level == 5 {
            return 1800
        }
        if self.level == 6 {
            return 1500
        }
        if self.level == 7 {
            return 1200
        }
        if self.level == 8 {
            return 900
        }
        if self.level == 9 {
            return 600
        }
        if self.level == 10 {
            return 300
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for FirePillar {
}
// WZ_SIGHTRASHER
pub struct Sightrasher {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Sightrasher {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        81
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
            if status.sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 43 { return Ok(43) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 47 { return Ok(47) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 51 { return Ok(51) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 53 { return Ok(53) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Sight
            if status.state & 1048576 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       500
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
impl SelfSkillBase for Sightrasher {
}
// WZ_METEOR
pub struct MeteorStorm {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MeteorStorm {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        83
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
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 54 { return Ok(54) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 64 { return Ok(64) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       15000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 4000
        }
        if self.level == 6 {
            return 5000
        }
        if self.level == 7 {
            return 5000
        }
        if self.level == 8 {
            return 6000
        }
        if self.level == 9 {
            return 6000
        }
        if self.level == 10 {
            return 7000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 4000
        }
        if self.level == 6 {
            return 5000
        }
        if self.level == 7 {
            return 5000
        }
        if self.level == 8 {
            return 6000
        }
        if self.level == 9 {
            return 6000
        }
        if self.level == 10 {
            return 7000
        }
        0
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for MeteorStorm {
}
// WZ_JUPITEL
pub struct JupitelThunder {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for JupitelThunder {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        84
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
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 47 { return Ok(47) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 2500
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 3500
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 4500
        }
        if self.level == 6 {
            return 5000
        }
        if self.level == 7 {
            return 5500
        }
        if self.level == 8 {
            return 6000
        }
        if self.level == 9 {
            return 6500
        }
        if self.level == 10 {
            return 7000
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
impl OffensiveSkillBase for JupitelThunder {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        if self.level == 1 {
            return 3
        }
        if self.level == 2 {
            return 4
        }
        if self.level == 3 {
            return 5
        }
        if self.level == 4 {
            return 6
        }
        if self.level == 5 {
            return 7
        }
        if self.level == 6 {
            return 8
        }
        if self.level == 7 {
            return 9
        }
        if self.level == 8 {
            return 10
        }
        if self.level == 9 {
            return 11
        }
        if self.level == 10 {
            return 12
        }
        0
    }
}
// WZ_VERMILION
pub struct LordofVermilion {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for LordofVermilion {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        85
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
            if status.sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 68 { return Ok(68) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 72 { return Ok(72) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 76 { return Ok(76) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 84 { return Ok(84) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 88 { return Ok(88) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 92 { return Ok(92) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 96 { return Ok(96) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 15000
        }
        if self.level == 2 {
            return 14500
        }
        if self.level == 3 {
            return 14000
        }
        if self.level == 4 {
            return 13500
        }
        if self.level == 5 {
            return 13000
        }
        if self.level == 6 {
            return 12500
        }
        if self.level == 7 {
            return 12000
        }
        if self.level == 8 {
            return 11500
        }
        if self.level == 9 {
            return 11000
        }
        if self.level == 10 {
            return 10500
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       5000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for LordofVermilion {
}
// WZ_WATERBALL
pub struct WaterBall {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for WaterBall {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        86
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
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_state(&self, status: &Status) -> SkillRequirementResult<()> {
        if status.state > 0 {
            // Water
            if status.state & 256 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 2000
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 5000
        }
        if self.level == 6 {
            return 6000
        }
        if self.level == 7 {
            return 7000
        }
        if self.level == 8 {
            return 8000
        }
        if self.level == 9 {
            return 9000
        }
        if self.level == 10 {
            return 10000
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
impl OffensiveSkillBase for WaterBall {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// WZ_ICEWALL
pub struct IceWall {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for IceWall {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        87
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
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for IceWall {
}
// WZ_FROSTNOVA
pub struct FrostNova {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FrostNova {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        88
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
            if status.sp >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 43 { return Ok(43) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 33 { return Ok(33) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 31 { return Ok(31) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 27 { return Ok(27) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 6000
        }
        if self.level == 2 {
            return 6000
        }
        if self.level == 3 {
            return 5500
        }
        if self.level == 4 {
            return 5500
        }
        if self.level == 5 {
            return 5000
        }
        if self.level == 6 {
            return 5000
        }
        if self.level == 7 {
            return 4500
        }
        if self.level == 8 {
            return 4500
        }
        if self.level == 9 {
            return 4000
        }
        if self.level == 10 {
            return 4000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
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
impl SelfSkillBase for FrostNova {
}
// WZ_STORMGUST
pub struct StormGust {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for StormGust {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        89
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
        if status.sp > 78 { Ok(78) } else {Err(())}
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 6000
        }
        if self.level == 2 {
            return 7000
        }
        if self.level == 3 {
            return 8000
        }
        if self.level == 4 {
            return 9000
        }
        if self.level == 5 {
            return 10000
        }
        if self.level == 6 {
            return 11000
        }
        if self.level == 7 {
            return 12000
        }
        if self.level == 8 {
            return 13000
        }
        if self.level == 9 {
            return 14000
        }
        if self.level == 10 {
            return 15000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       5000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for StormGust {
}
// WZ_EARTHSPIKE
pub struct EarthSpike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for EarthSpike {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        90
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
            if status.sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 2000
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 5000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       700
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
impl OffensiveSkillBase for EarthSpike {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        if self.level == 1 {
            return 1
        }
        if self.level == 2 {
            return 2
        }
        if self.level == 3 {
            return 3
        }
        if self.level == 4 {
            return 4
        }
        if self.level == 5 {
            return 5
        }
        0
    }
}
// WZ_HEAVENDRIVE
pub struct HeavensDrive {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for HeavensDrive {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        91
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
            if status.sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 44 { return Ok(44) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 2000
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 5000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       700
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for HeavensDrive {
}
// WZ_QUAGMIRE
pub struct Quagmire {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Quagmire {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        92
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
            if status.sp >= 5 { return Ok(5) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for Quagmire {
}
// WZ_ESTIMATION
pub struct Sense {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Sense {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        93
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
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for Sense {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// WZ_SIGHTBLASTER
pub struct SightBlaster {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SightBlaster {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        1006
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
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
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
impl SelfSkillBase for SightBlaster {
}
