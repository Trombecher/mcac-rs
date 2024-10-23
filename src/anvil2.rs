use crate::anvil::{EnchantmentKind, ItemKind};

#[derive(Copy, Clone, PartialEq)]
pub struct Enchantments(u128);

impl Enchantments {
    #[inline]
    pub const fn empty() -> Self {
        Enchantments(0)
    }
    
    #[inline]
    pub const fn set_level(self, kind: EnchantmentKind, level: u8) -> Self {
        let factor = (kind as u8 as u128) << 3;
        Self((self.0 & !(7 << factor)) | (((level & 7) as u128) << factor))
    }

    #[inline]
    pub const fn get_level(self, kind: EnchantmentKind) -> u8 {
        ((self.0 >> ((kind as u8 as u128) << 3)) & 7) as u8
    }

    #[inline]
    pub const fn has(self, kind: EnchantmentKind) -> bool {
        self.0 & (7 << ((kind as u8 as u128) << 3)) != 0
    }
    
    pub fn iter() -> impl Iterator<Item = u8> {
        
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Item {
    pub enchantments: Enchantments,
    pub prior_work_penalty: u16,
    pub kind: ItemKind,
}