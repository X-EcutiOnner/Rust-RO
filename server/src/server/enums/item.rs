#[derive(r#enum::WithNumberValue, Debug)]
pub enum ItemType {
    #[value = 0]
    Healing,
    Unknown,
    Usable,
    Etc,
    Armor,
    Weapon,
    Card,
    PetEgg,
    PetArmor,
    Unknown2,
    Ammo,
    DelayConsume,
    ShadowGear,
    #[value = 18]
    Cash,
    Max
}

impl ItemType {
    pub fn from_string(value: &str) -> Self {
        match value {
            "Healing" => Self::Healing,
            "Unknown" => Self::Unknown,
            "Usable" => Self::Usable,
            "Etc" => Self::Etc,
            "Armor" => Self::Armor,
            "Weapon" => Self::Weapon,
            "Card" => Self::Card,
            "PetEgg" => Self::PetEgg,
            "PetArmor" => Self::PetArmor,
            "Unknown2" => Self::Unknown2,
            "Ammo" => Self::Ammo,
            "DelayConsume" => Self::DelayConsume,
            "ShadowGear" => Self::ShadowGear,
            "Cash" => Self::Cash,
            "Max" => Self::Max,
            &_ => Self::Unknown
        }
    }
}