#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ItemKind {
    // Armor
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Elytra,

    // Tool
    Axe,
    Pickaxe,
    Shovel,
    Hoe,

    // Weapon
    Sword,
    Bow,
    Crossbow,
    Trident,
    Shield,

    // Misc
    Shears,
    FishingRod,
    FlintAndSteel,
    CarrotOnAStick,
    WarpedFungusOnAStick,
    Compass,
    Book,
    CarvedPumpkin,
    Head,
    RecoveryCompass,
}

impl ItemKind {
    pub const LAST: ItemKind = ItemKind::RecoveryCompass;
}