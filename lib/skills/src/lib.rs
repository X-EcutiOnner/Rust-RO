use enums::skill::{SkillTargetType, UseSkillFailure};
use enums::weapon::{AmmoType};
use models::item::{NormalInventoryItem};
use models::status::Status;

pub mod skill_enums;
pub mod base;
pub mod skills;

type SkillRequirementResult<T> = std::result::Result<T, ()>;

pub trait SkillBase {
    fn _level(&self) -> u8;
    fn _id(&self) -> u32;
    #[inline(always)]
    fn _validate_sp(&self, _status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_hp(&self, _status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_state(&self, _status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_zeny(&self, _status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_spirit_sphere(&self, _status: &Status) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }

    #[inline(always)]
    fn _validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, _status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_range(&self, _status: &Status) -> SkillRequirementResult<()> {
        Ok(())
    }

    #[inline(always)]
    fn _skip_item_validation(&self, _state: Option<u64>) -> bool {
        false
    }

    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        0
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }

    #[inline(always)]
    fn _update_cast_time(&mut self, _new_value: u32) {}
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, _new_value: u32) {}
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, _new_value: u32) {}

    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        0
    }

    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        None
    }
}

pub trait Skill: SkillBase {
    fn as_base(&self) -> &dyn SkillBase where Self: Sized {
        self as &dyn SkillBase
    }
    fn as_base_mut(&mut self) -> &mut dyn SkillBase where Self: Sized {
        self as &mut dyn SkillBase
    }

    fn new(level: u8) -> Option<Self> where Self: Sized;
    fn level(&self) -> u8 {
        self._level()
    }
    fn id(&self) -> u32 {
        self._id()
    }

    #[inline(always)]
    fn validate_sp(&self, _status: &Status) -> SkillRequirementResult<u32> {
        self._validate_sp(_status)
    }
    #[inline(always)]
    fn validate_hp(&self, _status: &Status) -> SkillRequirementResult<u32> {
        self._validate_hp(_status)
    }
    #[inline(always)]
    fn validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        self._validate_ammo(_character_ammo)
    }
    #[inline(always)]
    fn validate_state(&self, _status: &Status) -> SkillRequirementResult<()>  {
        self._validate_state(_status)
    }
    #[inline(always)]
    fn validate_zeny(&self, _status: &Status) -> SkillRequirementResult<u32>  {
        self._validate_zeny(_status)
    }
    #[inline(always)]
    fn validate_spirit_sphere(&self, _status: &Status) -> SkillRequirementResult<u32> {
        self._validate_spirit_sphere(_status)
    }
    #[inline(always)]
    fn validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        self._validate_item(_item)
    }

    #[inline(always)]
    fn validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        self._validate_target(_target_type)
    }
    #[inline(always)]
    fn validate_weapon(&self, _status: &Status) -> SkillRequirementResult<()> {
        self._validate_weapon(_status)
    }
    #[inline(always)]
    fn validate_range(&self, _status: &Status) -> SkillRequirementResult<()> {
        self._validate_range(_status)
    }

    #[inline(always)]
    fn skip_item_validation(&self, _state: Option<u64>) -> bool  {
        self._skip_item_validation(_state)
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        self._hit_count()
    }
    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        self._base_cast_time()
    }
    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32{
        self._base_after_cast_act_delay()
    }
    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
        self._base_after_cast_walk_delay()
    }

    #[inline(always)]
    fn update_cast_time(&mut self, _new_value: u32)  {
        self._update_cast_time(_new_value)
    }
    #[inline(always)]
    fn update_after_cast_act_delay(&mut self, _new_value: u32) {
        self._update_after_cast_act_delay(_new_value)
    }
    #[inline(always)]
    fn update_after_cast_walk_delay(&mut self, _new_value: u32)  {
        self._update_after_cast_walk_delay(_new_value)
    }

    #[inline(always)]
    fn cast_time(&self) -> u32 {
        self._cast_time()
    }
    #[inline(always)]
    fn after_cast_act_delay(&self) -> u32  {
        self._after_cast_act_delay()
    }
    #[inline(always)]
    fn after_cast_walk_delay(&self) -> u32 {
        self._after_cast_walk_delay()
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        self._dmg_atk()
    }
}
