use crate::*;

#[derive(WithNumberValue, WithStringValue, Debug, PartialEq)]
pub enum JobName {
    #[value = 0]
    Novice,
    Swordsman,
    Mage,
    Archer,
    Acolyte,
    Merchant,
    Thief,
    Knight,
    Priest,
    Wizard,
    Blacksmith,
    Hunter,
    Assassin,
    Crusader,
    Monk,
    Sage,
    Rogue,
    Alchemist,
    Bard,
    Dancer,
    Wedding,
    #[value = 23]
    SuperNovice,
    Gunslinger,
    Ninja,
    Xmas,
    Summer,
    #[value_string = "Novice High"]
    #[value = 4001]
    NoviceHigh,
    #[value_string = "Swordsman High"]
    SwordsmanHigh,
    #[value_string = "Mage High"]
    MageHigh,
    #[value_string = "Archer High"]
    ArcherHigh,
    #[value_string = "Acolyte High"]
    AcolyteHigh,
    #[value_string = "Merchant High"]
    MerchantHigh,
    #[value_string = "Thief High"]
    ThiefHigh,
    LordKnight,
    HighPriest,
    HighWizard,
    Whitesmith,
    Sniper,
    AssassinCross,
    #[value = 4015]
    Paladin,
    Champion,
    Professor,
    Stalker,
    Creator,
    Clown,
    Gypsy,
    #[value = 4023]
    BabyNovice,
    BabySwordman,
    BabyMage,
    BabyArcher,
    BabyAcolyte,
    BabyMerchant,
    BabyThief,
    BabyKnight,
    BabyPriest,
    BabyWizard,
    BabyBlacksmith,
    BabyHunter,
    BabyAssassin,
    BabyCrusader,
    BabyMonk,
    BabySage,
    BabyRogue,
    BabyAlchemist,
    #[value = 4045]
    SuperBaby,
    Taekwon,
    StarGladiator,
    SoulLinker,
}

impl JobName {
    pub fn is_rebirth(&self) -> bool {
        match self {
            JobName::NoviceHigh | JobName::SwordsmanHigh | JobName::MageHigh | JobName::ArcherHigh
            | JobName::AcolyteHigh | JobName::MerchantHigh | JobName::ThiefHigh | JobName::LordKnight | JobName::HighPriest
            | JobName::HighWizard | JobName::Whitesmith | JobName::Sniper | JobName::AssassinCross | JobName::Paladin
            | JobName::Champion | JobName::Professor | JobName::Stalker | JobName::Creator | JobName::Clown | JobName::Gypsy => true,
            _ => false,
        }
    }
    pub fn is_novice(&self) -> bool {
        match self {
            JobName::NoviceHigh | JobName::Novice | JobName::SuperNovice => true,
            _ => false
        }
    }
    pub fn is_first_class(&self) -> bool {
        match self {
            JobName::Swordsman | JobName::Mage | JobName::Archer | JobName::Acolyte
            | JobName::Merchant | JobName::Thief | JobName::BabySwordman | JobName::BabyMage
            | JobName::BabyArcher | JobName::BabyAcolyte | JobName::BabyMerchant | JobName::BabyThief
            | JobName::SwordsmanHigh | JobName::MageHigh | JobName::ArcherHigh
            | JobName::AcolyteHigh | JobName::MerchantHigh | JobName::ThiefHigh => true,
            _ => false,
        }
    }
    pub fn is_second_class(&self) -> bool {
        match self {
            JobName::Knight | JobName::Priest | JobName::Wizard
            | JobName::Blacksmith | JobName::Hunter | JobName::Assassin | JobName::Crusader | JobName::Monk
            | JobName::Sage | JobName::Rogue | JobName::Alchemist | JobName::Bard | JobName::Dancer
            | JobName::BabyKnight | JobName::BabyPriest | JobName::BabyWizard | JobName::BabyBlacksmith
            | JobName::BabyHunter | JobName::BabyAssassin | JobName::BabyCrusader | JobName::BabyMonk
            | JobName::BabySage | JobName::BabyRogue | JobName::BabyAlchemist
            | JobName::LordKnight | JobName::HighPriest
            | JobName::HighWizard | JobName::Whitesmith | JobName::Sniper | JobName::AssassinCross | JobName::Paladin
            | JobName::Champion | JobName::Professor | JobName::Stalker | JobName::Creator | JobName::Clown | JobName::Gypsy => true,
            _ => false,
        }
    }
    pub fn is_gunslinger_ninja(&self) -> bool {
        match self {
            JobName::Gunslinger | JobName::Ninja => true,
            _ => false,
        }
    }
    pub fn is_taekwon(&self) -> bool {
        match self {
            JobName::Taekwon | JobName::StarGladiator | JobName::SoulLinker => true,
            _ => false,
        }
    }
}

#[derive(WithStringValue, WithMaskValueU64)]
pub enum EquipClassFlag {
    #[mask_value = 1]
    Novice,
    Acolyte,
    Priest,
    Monk,
    Mage,
    Wizard,
    Sage,
    Thief,
    Assassin,
    Rogue,
    Archer,
    Hunter,
    Bard,
    Dancer,
    Swordsman,
    Crusader,
    Knight,
    Merchant,
    Alchemist,
    Blacksmith,
    Gunslinger,
    Ninja,
    SoulLinker,
    StarGladiator,
    SuperNovice,
    Taekwon,
    #[mask_all]
    All,
}

impl EquipClassFlag {
    pub fn flag_from_job_name(job_name: JobName) -> u64 {
        match job_name {
            JobName::Novice => EquipClassFlag::Novice.as_flag(),
            JobName::Acolyte => EquipClassFlag::Acolyte.as_flag(),
            JobName::Priest => EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Monk => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Mage => EquipClassFlag::Mage.as_flag(),
            JobName::Wizard => EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Sage => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Thief => EquipClassFlag::Thief.as_flag(),
            JobName::Assassin => EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Rogue => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Archer => EquipClassFlag::Archer.as_flag(),
            JobName::Hunter => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Bard => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Dancer => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Swordsman => EquipClassFlag::Swordsman.as_flag(),
            JobName::Crusader => EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::Knight => EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::Merchant => EquipClassFlag::Merchant.as_flag(),
            JobName::Alchemist => EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Blacksmith => EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Gunslinger => EquipClassFlag::Gunslinger.as_flag(),
            JobName::Ninja => EquipClassFlag::Ninja.as_flag(),
            JobName::SoulLinker => EquipClassFlag::SoulLinker.as_flag(),
            JobName::StarGladiator => EquipClassFlag::StarGladiator.as_flag(),
            JobName::SuperNovice => EquipClassFlag::SuperNovice.as_flag(),
            JobName::Taekwon => EquipClassFlag::Taekwon.as_flag(),
            JobName::Wedding => 0,
            JobName::Xmas => 0,
            JobName::Summer => 0,
            JobName::NoviceHigh => EquipClassFlag::Novice.as_flag(),
            JobName::SwordsmanHigh => EquipClassFlag::Swordsman.as_flag(),
            JobName::MageHigh => EquipClassFlag::Mage.as_flag(),
            JobName::ArcherHigh => EquipClassFlag::Archer.as_flag(),
            JobName::AcolyteHigh => EquipClassFlag::Acolyte.as_flag(),
            JobName::MerchantHigh => EquipClassFlag::Merchant.as_flag(),
            JobName::ThiefHigh => EquipClassFlag::Thief.as_flag(),
            JobName::HighPriest => EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::HighWizard => EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Whitesmith => EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Sniper => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::AssassinCross => EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Champion => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Professor => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Stalker => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Creator => EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Clown => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Gypsy => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Paladin => EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::LordKnight => EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyNovice => EquipClassFlag::Novice.as_flag(),
            JobName::BabySwordman => EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyMage => EquipClassFlag::Mage.as_flag(),
            JobName::BabyArcher => EquipClassFlag::Archer.as_flag(),
            JobName::BabyAcolyte => EquipClassFlag::Acolyte.as_flag(),
            JobName::BabyMerchant => EquipClassFlag::Merchant.as_flag(),
            JobName::BabyThief => EquipClassFlag::Thief.as_flag(),
            JobName::BabyKnight => EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyPriest => EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::BabyWizard => EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::BabyBlacksmith => EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::BabyHunter => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::BabyAssassin => EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::BabyCrusader => EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyMonk => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::BabySage => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::BabyRogue => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::BabyAlchemist => EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::SuperBaby => 0,
        }
    }
}