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
// BS_IRON
pub struct IronTempering {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for IronTempering {
    fn _id(&self) -> u32 {
        94
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
// BS_STEEL
pub struct SteelTempering {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SteelTempering {
    fn _id(&self) -> u32 {
        95
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
// BS_ENCHANTEDSTONE
pub struct EnchantedStoneCraft {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for EnchantedStoneCraft {
    fn _id(&self) -> u32 {
        96
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
// BS_ORIDEOCON
pub struct OrideconResearch {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for OrideconResearch {
    fn _id(&self) -> u32 {
        97
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
// BS_DAGGER
pub struct SmithDagger {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithDagger {
    fn _id(&self) -> u32 {
        98
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
// BS_SWORD
pub struct SmithSword {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithSword {
    fn _id(&self) -> u32 {
        99
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
// BS_TWOHANDSWORD
pub struct SmithTwohandedSword {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithTwohandedSword {
    fn _id(&self) -> u32 {
        100
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
// BS_AXE
pub struct SmithAxe {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithAxe {
    fn _id(&self) -> u32 {
        101
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
// BS_MACE
pub struct SmithMace {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithMace {
    fn _id(&self) -> u32 {
        102
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
// BS_KNUCKLE
pub struct SmithKnucklebrace {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithKnucklebrace {
    fn _id(&self) -> u32 {
        103
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
// BS_SPEAR
pub struct SmithSpear {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SmithSpear {
    fn _id(&self) -> u32 {
        104
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
// BS_HILTBINDING
pub struct HiltBinding {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for HiltBinding {
    fn _id(&self) -> u32 {
        105
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
// BS_FINDINGORE
pub struct OreDiscovery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for OreDiscovery {
    fn _id(&self) -> u32 {
        106
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
// BS_WEAPONRESEARCH
pub struct WeaponryResearch {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for WeaponryResearch {
    fn _id(&self) -> u32 {
        107
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
// BS_REPAIRWEAPON
pub struct WeaponRepair {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for WeaponRepair {
    fn _id(&self) -> u32 {
        108
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
       7500
    }
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// BS_SKINTEMPER
pub struct SkinTempering {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SkinTempering {
    fn _id(&self) -> u32 {
        109
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
// BS_HAMMERFALL
pub struct HammerFall {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for HammerFall {
    fn _id(&self) -> u32 {
        110
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
            if 454 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
// BS_ADRENALINE
pub struct AdrenalineRush {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AdrenalineRush {
    fn _id(&self) -> u32 {
        111
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
            if 448 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
// BS_WEAPONPERFECT
pub struct WeaponPerfection {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for WeaponPerfection {
    fn _id(&self) -> u32 {
        112
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
            if status.sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 10 { return Ok(10) } else {return Err(())}
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
// BS_OVERTHRUST
pub struct PowerThrust {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for PowerThrust {
    fn _id(&self) -> u32 {
        113
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
            if status.sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 10 { return Ok(10) } else {return Err(())}
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
            if 25165822 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
// BS_MAXIMIZE
pub struct MaximizePower {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MaximizePower {
    fn _id(&self) -> u32 {
        114
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
// BS_UNFAIRLYTRICK
pub struct UnfairTrick {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for UnfairTrick {
    fn _id(&self) -> u32 {
        1012
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
// BS_GREED
pub struct Greed {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Greed {
    fn _id(&self) -> u32 {
        1013
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
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
    }
}
// BS_ADRENALINE2
pub struct AdvancedAdrenalineRush {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AdvancedAdrenalineRush {
    fn _id(&self) -> u32 {
        459
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
        if status.sp > 64 { Ok(64) } else {Err(())}
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
            if 129023 & character_weapon.weapon_type.as_flag() > 0 { Ok(()) } else { Err(()) }
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
