use crate::bits::{EnchantmentMask, ItemMask};
use crate::items::ItemKind;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enchantment {
    kind: EnchantmentKind,
    level: u8,
}

impl Enchantment {
    #[inline]
    pub const fn new(kind: EnchantmentKind, level: u8) -> Option<Self> {
        if level > kind.max_level() {
            None
        } else {
            Some(Self { kind, level })
        }
    }

    #[inline]
    pub const fn kind(self) -> EnchantmentKind {
        self.kind
    }

    #[inline]
    pub const fn level(self) -> u8 {
        self.level
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum EnchantmentKind {
    // General
    Mending,
    Unbreaking,
    CurseOfBinding,
    CurseOfVanishing,

    // Armor
    Protection,
    BlastProtection,
    FireProtection,
    ProjectileProtection,
    Thorns,

    // Helmet
    Respiration,
    AquaAffinity,

    // Leggings
    SwiftSneak,

    // Boots
    DepthStrider,
    FrostWalker,
    FeatherFalling,
    SoulSpeed,

    // Weapon
    SweepingEdge,
    Sharpness,
    Smite,
    BaneOfArthropods,
    Knockback,
    FireAspect,
    Looting,

    // Tool
    SilkTouch,
    Efficiency,
    Fortune,

    // Bow
    Power,
    Punch,
    Flame,
    Infinity,

    // Fishing Rod
    LuckOfTheSea,
    Lure,

    // Trident
    Impaling,
    Riptide,
    Loyalty,
    Channeling,

    // Crossbow
    Multishot,
    Piercing,
    QuickCharge,
}

impl EnchantmentKind {
    pub const fn max_level(self) -> u8 {
        match self {
            EnchantmentKind::Mending => 2,
            EnchantmentKind::Unbreaking => 3,
            EnchantmentKind::CurseOfBinding => 1,
            EnchantmentKind::CurseOfVanishing => 1,
            EnchantmentKind::Protection => 4,
            EnchantmentKind::BlastProtection => 4,
            EnchantmentKind::FireProtection => 4,
            EnchantmentKind::ProjectileProtection => 4,
            EnchantmentKind::Thorns => 3,
            EnchantmentKind::Respiration => 3,
            EnchantmentKind::AquaAffinity => 1,
            EnchantmentKind::SwiftSneak => 3,
            EnchantmentKind::DepthStrider => 3,
            EnchantmentKind::FrostWalker => 2,
            EnchantmentKind::FeatherFalling => 4,
            EnchantmentKind::SoulSpeed => 3,
            EnchantmentKind::SweepingEdge => 3,
            EnchantmentKind::Sharpness => 5,
            EnchantmentKind::Smite => 5,
            EnchantmentKind::BaneOfArthropods => 5,
            EnchantmentKind::Knockback => 2,
            EnchantmentKind::FireAspect => 2,
            EnchantmentKind::Looting => 3,
            EnchantmentKind::SilkTouch => 1,
            EnchantmentKind::Efficiency => 5,
            EnchantmentKind::Fortune => 3,
            EnchantmentKind::Power => 5,
            EnchantmentKind::Punch => 2,
            EnchantmentKind::Flame => 1,
            EnchantmentKind::Infinity => 1,
            EnchantmentKind::LuckOfTheSea => 3,
            EnchantmentKind::Lure => 3,
            EnchantmentKind::Impaling => 5,
            EnchantmentKind::Riptide => 3,
            EnchantmentKind::Loyalty => 3,
            EnchantmentKind::Channeling => 1,
            EnchantmentKind::Multishot => 1,
            EnchantmentKind::Piercing => 4,
            EnchantmentKind::QuickCharge => 3,
        }
    }

    pub fn applicable_to(self) -> ItemMask {
        match self {
            EnchantmentKind::Mending => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
                ItemKind::FishingRod,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
                ItemKind::Bow,
                ItemKind::Shears,
                ItemKind::FlintAndSteel,
                ItemKind::CarrotOnAStick,
                ItemKind::WarpedFungusOnAStick,
                ItemKind::Shield,
                ItemKind::Elytra,
                ItemKind::Trident,
                ItemKind::Crossbow,
            ]),
            EnchantmentKind::Unbreaking => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
                ItemKind::FishingRod,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
                ItemKind::Bow,
                ItemKind::Shears,
                ItemKind::FlintAndSteel,
                ItemKind::CarrotOnAStick,
                ItemKind::WarpedFungusOnAStick,
                ItemKind::Shield,
                ItemKind::Elytra,
                ItemKind::Trident,
                ItemKind::Crossbow
            ]),
            EnchantmentKind::CurseOfBinding => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
                ItemKind::Elytra,
                ItemKind::CarvedPumpkin,
                ItemKind::Head
            ]),
            EnchantmentKind::CurseOfVanishing => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
                ItemKind::FishingRod,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
                ItemKind::Bow,
                ItemKind::Shears,
                ItemKind::FlintAndSteel,
                ItemKind::CarrotOnAStick,
                ItemKind::WarpedFungusOnAStick,
                ItemKind::Shield,
                ItemKind::Elytra,
                ItemKind::Trident,
                ItemKind::Crossbow,
                ItemKind::CarvedPumpkin,
                ItemKind::Head,
                ItemKind::Compass,
                ItemKind::RecoveryCompass
            ]),
            EnchantmentKind::Protection => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ]),
            EnchantmentKind::BlastProtection => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ]),
            EnchantmentKind::FireProtection => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ]),
            EnchantmentKind::ProjectileProtection => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ]),
            EnchantmentKind::Thorns => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
            ]),
            EnchantmentKind::Respiration => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet,
            ]),
            EnchantmentKind::AquaAffinity => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Helmet
            ]),
            EnchantmentKind::SwiftSneak => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Leggings
            ]),
            EnchantmentKind::DepthStrider => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Boots
            ]),
            EnchantmentKind::FrostWalker => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Boots
            ]),
            EnchantmentKind::FeatherFalling => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Boots
            ]),
            EnchantmentKind::SoulSpeed => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Boots
            ]),
            EnchantmentKind::SweepingEdge => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Axe,
            ]),
            EnchantmentKind::Sharpness => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
                ItemKind::Axe,
            ]),
            EnchantmentKind::Smite => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
                ItemKind::Axe,
            ]),
            EnchantmentKind::BaneOfArthropods => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
                ItemKind::Axe,
            ]),
            EnchantmentKind::Knockback => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword
            ]),
            EnchantmentKind::FireAspect => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword,
            ]),
            EnchantmentKind::Looting => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Sword
            ]),
            EnchantmentKind::SilkTouch => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
            ]),
            EnchantmentKind::Efficiency => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
                ItemKind::Shears
            ]),
            EnchantmentKind::Fortune => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
            ]),
            EnchantmentKind::Power => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Bow
            ]),
            EnchantmentKind::Punch => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Bow
            ]),
            EnchantmentKind::Flame => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Bow
            ]),
            EnchantmentKind::Infinity => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Bow
            ]),
            EnchantmentKind::LuckOfTheSea => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::FishingRod
            ]),
            EnchantmentKind::Lure => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::FishingRod
            ]),
            EnchantmentKind::Impaling => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Trident
            ]),
            EnchantmentKind::Riptide => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Trident
            ]),
            EnchantmentKind::Loyalty => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Trident
            ]),
            EnchantmentKind::Channeling => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Trident
            ]),
            EnchantmentKind::Multishot => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Crossbow
            ]),
            EnchantmentKind::Piercing => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Crossbow
            ]),
            EnchantmentKind::QuickCharge => ItemMask::from_slice(&[
                ItemKind::Book,
                ItemKind::Crossbow
            ])
        }
    }

    pub fn incompatible_with(self) -> EnchantmentMask {
        match self {
            EnchantmentKind::Mending => EnchantmentMask::from_slice(&[
                EnchantmentKind::Infinity
            ]),
            EnchantmentKind::Unbreaking => EnchantmentMask::empty(),
            EnchantmentKind::CurseOfBinding => EnchantmentMask::empty(),
            EnchantmentKind::CurseOfVanishing => EnchantmentMask::empty(),
            EnchantmentKind::Protection => EnchantmentMask::from_slice(&[
                EnchantmentKind::FireProtection,
                EnchantmentKind::BlastProtection,
                EnchantmentKind::ProjectileProtection,
            ]),
            EnchantmentKind::BlastProtection => EnchantmentMask::from_slice(&[
                EnchantmentKind::FireProtection,
                EnchantmentKind::Protection,
                EnchantmentKind::ProjectileProtection,
            ]),
            EnchantmentKind::FireProtection => EnchantmentMask::from_slice(&[
                EnchantmentKind::Protection,
                EnchantmentKind::BlastProtection,
                EnchantmentKind::ProjectileProtection,
            ]),
            EnchantmentKind::ProjectileProtection => EnchantmentMask::from_slice(&[
                EnchantmentKind::FireProtection,
                EnchantmentKind::BlastProtection,
                EnchantmentKind::Protection,
            ]),
            EnchantmentKind::Thorns => EnchantmentMask::empty(),
            EnchantmentKind::Respiration => EnchantmentMask::empty(),
            EnchantmentKind::AquaAffinity => EnchantmentMask::empty(),
            EnchantmentKind::SwiftSneak => EnchantmentMask::empty(),
            EnchantmentKind::DepthStrider => EnchantmentMask::from_slice(&[
                EnchantmentKind::FrostWalker
            ]),
            EnchantmentKind::FrostWalker => EnchantmentMask::from_slice(&[
                EnchantmentKind::DepthStrider
            ]),
            EnchantmentKind::FeatherFalling => EnchantmentMask::empty(),
            EnchantmentKind::SoulSpeed => EnchantmentMask::empty(),
            EnchantmentKind::SweepingEdge => EnchantmentMask::empty(),
            EnchantmentKind::Sharpness => EnchantmentMask::from_slice(&[
                EnchantmentKind::Smite,
                EnchantmentKind::BaneOfArthropods,
            ]),
            EnchantmentKind::Smite => EnchantmentMask::from_slice(&[
                EnchantmentKind::Sharpness,
                EnchantmentKind::BaneOfArthropods,
            ]),
            EnchantmentKind::BaneOfArthropods => EnchantmentMask::from_slice(&[
                EnchantmentKind::Sharpness,
                EnchantmentKind::Smite,
            ]),
            EnchantmentKind::Knockback => EnchantmentMask::empty(),
            EnchantmentKind::FireAspect => EnchantmentMask::empty(),
            EnchantmentKind::Looting => EnchantmentMask::empty(),
            EnchantmentKind::SilkTouch => EnchantmentMask::from_slice(&[
                EnchantmentKind::Fortune
            ]),
            EnchantmentKind::Efficiency => EnchantmentMask::empty(),
            EnchantmentKind::Fortune => EnchantmentMask::from_slice(&[
                EnchantmentKind::SilkTouch
            ]),
            EnchantmentKind::Power => EnchantmentMask::empty(),
            EnchantmentKind::Punch => EnchantmentMask::empty(),
            EnchantmentKind::Flame => EnchantmentMask::empty(),
            EnchantmentKind::Infinity => EnchantmentMask::from_slice(&[
                EnchantmentKind::Mending
            ]),
            EnchantmentKind::LuckOfTheSea => EnchantmentMask::empty(),
            EnchantmentKind::Lure => EnchantmentMask::empty(),
            EnchantmentKind::Impaling => EnchantmentMask::empty(),
            EnchantmentKind::Riptide => EnchantmentMask::from_slice(&[
                EnchantmentKind::Loyalty,
                EnchantmentKind::Channeling
            ]),
            EnchantmentKind::Loyalty => EnchantmentMask::from_slice(&[
                EnchantmentKind::Riptide
            ]),
            EnchantmentKind::Channeling => EnchantmentMask::from_slice(&[
                EnchantmentKind::Riptide
            ]),
            EnchantmentKind::Multishot => EnchantmentMask::from_slice(&[
                EnchantmentKind::Piercing
            ]),
            EnchantmentKind::Piercing => EnchantmentMask::from_slice(&[
                EnchantmentKind::Multishot
            ]),
            EnchantmentKind::QuickCharge => EnchantmentMask::empty(),
        }
    }

    const ITEM_MULTIPLIERS: u128 = {
        let mut x = 0_u128;
        let mut i = 0;

        while i < 39 {
            x |= match [4, 2, 8, 8, 1, 4, 2, 2, 8, 4, 4, 8, 4, 4, 2, 8, 4, 1, 2, 2, 2, 4, 4, 8, 1, 4, 1, 4, 4, 8, 4, 4, 4, 4, 1, 8, 4, 1, 2][i] {
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                _ => unreachable!()
            } << (i * 2);

            i += 1;
        }

        x
    };

    pub fn item_multiplier(self) -> u8 {
        1 << ((Self::ITEM_MULTIPLIERS >> (self as u128 * 2)) & 3) as u8
    }

    pub const LAST: Self = Self::QuickCharge;

    #[inline]
    pub fn book_multiplier(self) -> u8 {
        let i = self.item_multiplier();
        if i == 1 {
            1
        } else {
            i / 2
        }
    }
}
