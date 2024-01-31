use crate::element::Element;

#[derive(Debug, Clone, Copy)]
pub enum BonusType {
    Str(i8),
    Agi(i8),
    Vit(i8),
    Int(i8),
    Dex(i8),
    Luk(i8),
    AllStats(i8),
    Hit(i8),
    HitPercentage(i8),
    Flee(i8),
    Crit(i8),
    PerfectDodge(i8),
    Aspd(i8),
    AspdPercentage(i8),
    Maxhp(i8),
    Maxsp(i8),
    MaxhpPercentage(i8),
    MaxspPercentage(i8),
    Atk(i8),
    Def(i8),
    VitDefPercentage(i8),
    DefPercentage(i8),
    Mdef(i8),
    Matk(i8),
    MatkBasedOnStaffPercentage(i8),
    MatkPercentage(i8),
    AtkPercentage(i8),
    PerfectHitPercentage(i8),
    ElementWeapon(Element),
    ElementDefense(Element),
    BypassDefenseOnRace,
    WeaponAtkIncreaseOnTargetDefense,
    ReduceDefense(i8),
    ReduceDefensePercentage(i8),
    CriticalDamagePercentage(i8),
    CastTimePercentage(i8),
    AfterCastDelayPercentage(i8),
    NaturalHpRecoveryPercentage(i8),
    NaturalSpRecoveryPercentage(i8),
    HpRegenPercentage(i8),
    SpRegenPercentage(i8),
    HpRegenFromItemPercentage(i8),
    GainHpWhenKillingEnemy(i8),
    GainHpWhenKillingEnemyWithMagicAttack(i8),
    GainSpWhenKillingEnemyWithMagicAttack(i8),
    HpRegenFromSkillPercentage(i8),
    DisableHpRegen,
    DisableSpRegen,
    GainSpWhenHittingEnemy(i8),
    GainSpWhenKillingEnemy(i8),
    SpConsumption(i8),
    ResistanceRangeAttackPercentage(i8),
    NormalAttackPercentage(i8),
    IncreaseDamageAgainstBossPercentage(i8),
    IncreaseDamageAgainstSizeSmallPercentage(i8),
    IncreaseDamageAgainstSizeMediumPercentage(i8),
    IncreaseDamageAgainstSizeLargePercentage(i8),
    IncreaseDamageRaceFormlessPercentage(i8),
    IncreaseDamageRaceUndeadPercentage(i8),
    IncreaseDamageRaceBrutePercentage(i8),
    IncreaseDamageRacePlantPercentage(i8),
    IncreaseDamageRaceInsectPercentage(i8),
    IncreaseDamageRaceFishPercentage(i8),
    IncreaseDamageRaceDemonPercentage(i8),
    IncreaseDamageRaceDemihumanPercentage(i8),
    IncreaseDamageRaceAngelPercentage(i8),
    IncreaseDamageRaceDragonPercentage(i8),
    IncreaseDamageElementNeutralPercentage(i8),
    IncreaseDamageElementWaterPercentage(i8),
    IncreaseDamageElementEarthPercentage(i8),
    IncreaseDamageElementFirePercentage(i8),
    IncreaseDamageElementWindPercentage(i8),
    IncreaseDamageElementPoisonPercentage(i8),
    IncreaseDamageElementHolyPercentage(i8),
    IncreaseDamageElementDarkPercentage(i8),
    IncreaseDamageElementGhostPercentage(i8),
    IncreaseDamageElementUndeadPercentage(i8),
    DamageIncDecRacePercentage(i8),
    DamageIncDecRaceFormlessPercentage(i8),
    DamageIncDecRaceUndeadPercentage(i8),
    DamageIncDecRaceBrutePercentage(i8),
    DamageIncDecRacePlantPercentage(i8),
    DamageIncDecRaceInsectPercentage(i8),
    DamageIncDecRaceFishPercentage(i8),
    DamageIncDecRaceDemonPercentage(i8),
    DamageIncDecRaceDemihumanPercentage(i8),
    DamageIncDecRaceAngelPercentage(i8),
    DamageIncDecRaceDragonPercentage(i8),
    DamageIncDecElementNeutralPercentage(i8),
    DamageIncDecElementWaterPercentage(i8),
    DamageIncDecElementEarthPercentage(i8),
    DamageIncDecElementFirePercentage(i8),
    DamageIncDecElementWindPercentage(i8),
    DamageIncDecElementPoisonPercentage(i8),
    DamageIncDecElementHolyPercentage(i8),
    DamageIncDecElementDarkPercentage(i8),
    DamageIncDecElementGhostPercentage(i8),
    DamageIncDecElementUndeadPercentage(i8),
    IncreaseDamageGoblinPercentage(i8),
    IncreaseDamageKoboldPercentage(i8),
    IncreaseDamageOrcPercentage(i8),
    IncreaseDamageGolemPercentage(i8),
    LowerDefencePercentage(i8),
    CriticalAgainstRaceFormlessPercentage(i8),
    CriticalAgainstRaceUndeadPercentage(i8),
    CriticalAgainstRaceBrutePercentage(i8),
    CriticalAgainstRacePlantPercentage(i8),
    CriticalAgainstRaceInsectPercentage(i8),
    CriticalAgainstRaceFishPercentage(i8),
    CriticalAgainstRaceDemonPercentage(i8),
    CriticalAgainstRaceDemihumanPercentage(i8),
    CriticalAgainstRaceAngelPercentage(i8),
    CriticalAgainstRaceDragonPercentage(i8),
    ChanceToInflictStatusOnAttack(i8),
    ChanceToInflictStatusPoisonOnAttack(i8),
    ChanceToInflictStatusStunOnAttack(i8),
    ChanceToInflictStatusFreezeOnAttack(i8),
    ChanceToInflictStatusCurseOnAttack(i8),
    ChanceToInflictStatusBlindOnAttack(i8),
    ChanceToInflictStatusSleepOnAttack(i8),
    ChanceToInflictStatusSilenceOnAttack(i8),
    ChanceToInflictStatusChaosOnAttack(i8),
    ChanceToInflictStatusBleedingOnAttack(i8),
    ChanceToInflictStatusStoneOnAttack(i8),
    ChanceToInflictStatusWeaponBreakOnAttack(i8),
    ChanceToInflictStatusArmorBreakOnAttack(i8),
    ResistanceToStatusPoisonPercentage(i8),
    ResistanceToStatusStunPercentage(i8),
    ResistanceToStatusFreezePercentage(i8),
    ResistanceToStatusCursePercentage(i8),
    ResistanceToStatusBlindPercentage(i8),
    ResistanceToStatusSleepPercentage(i8),
    ResistanceToStatusSilencePercentage(i8),
    ResistanceToStatusChaosPercentage(i8),
    ResistanceToStatusBleedingPercentage(i8),
    ResistanceToStatusStonePercentage(i8),
    ResistanceToStatusWeaponBreakPercentage(i8),
    ResistanceToStatusArmorBreakPercentage(i8),
    DamageIncDecSizePercentage(i8),
    DamageIncDecSizeSmallPercentage(i8),
    DamageIncDecSizeMediumPercentage(i8),
    DamageIncDecSizeLargePercentage(i8),
    BreakArmorPercentage(i8),
    BreakWeaponPercentage(i8),
    ClassChangePercentageOnHit(i8),
    IncreaseDecreaseCriticalChance(i8),
    IncreaseDecreaseLongRangeCriticalChance(i8),
    IncreaseDamageAgainstBossBaseOnDef,
    IncreaseDamageAgainstAllBaseOnDef,
    IncreaseDamageAgainstNormalBaseOnDef,
    IncreaseDamageAgainstGuardianBaseOnDef,
    SkillDelayIncDecPercentage(i8),
    DoubleAttackChancePercentage(i8),
    HealSkillPercentage(i8),
    IgnoreDefClassNormal,
    IgnoreDefClassAll,
    IgnoreDefClassBoss,
    IgnoreDefClassGuardian,
    IgnoreDefRaceAll,
    IgnoreDefRaceAngel,
    IgnoreDefRaceBrute,
    IgnoreDefRaceDemiHuman,
    IgnoreDefRaceDemon,
    IgnoreDefRaceDragon,
    IgnoreDefRaceFish,
    IgnoreDefRaceFormless,
    IgnoreDefRaceInsect,
    IgnoreDefRacePlant,
    IgnoreDefRacePlayerHuman,
    IgnoreDefRacePlayerDoram,
    IgnoreDefRaceUndead,
    ResistanceRangeAttack(i8),
    DamageRangedAtkPercentage(i8),
    ResistanceMagicAttackPercentage(i8),
    MagicAttackRelectChancePercentage(i8),
    MeleeAttackRelectChancePercentage(i8),
    SplashRadius(i8),
    SpeedPercentage(i8),
    EnableFullHpSpRecoverOnResurrect,
    EnableSeeHidden,
    EnableNoCancelCast,
    EnableNoGemstoneRequired,
    EnableIgnoreSizeModifier,
    EnableNoKnockback,
    EnableNoWalkDelay,
    UnbreakableArmor,
    UnbreakableShoulder,
    UnbreakableHelm,
    UnbreakableShield,
    UnbreakableShoes,
    UnbreakableWeapon,
}