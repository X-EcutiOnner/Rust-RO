#![allow(dead_code)]

use enum_macro::WithNumberValue;

#[derive(Debug, Clone, Copy, PartialEq, WithNumberValue)]
pub enum ActionType {
    Attack,
    Itempickup,
    Sit,
    Stand,
    AttackNomotion,
    Splash,
    Skill,
    AttackRepeat,
    AttackMultiple,
    AttackMultipleNomotion,
    AttackCritical,
    AttackLucky,
    Touchskill,
    AttackMultipleCritical,
}