use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use crate::enchantments::{Enchantment, EnchantmentKind};
use crate::items::ItemKind;

#[derive(Clone, Copy, PartialEq)]
pub struct ItemMask(u32);

impl Debug for ItemMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(self.iter().filter_map(|(kind, contained)| contained.then_some(kind)))
            .finish()
    }
}

impl ItemMask {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_slice(slice: &[ItemKind]) -> Self {
        let mut mask = Self::empty();
        let mut index = 0;

        while index < slice.len() {
            mask = mask.add(slice[index]);
            index += 1;
        }

        mask
    }

    #[inline]
    pub const fn has(self, kind: ItemKind) -> bool {
        self.0 & (1 << kind as u32) != 0
    }

    #[inline]
    pub const fn add(self, kind: ItemKind) -> Self {
        Self(self.0 | (1 << kind as u32))
    }

    #[inline]
    pub const fn remove(self, kind: ItemKind) -> Self {
        Self(self.0 & !(1 << kind as u32))
    }
    
    pub fn iter(self) -> impl Iterator<Item = (ItemKind, bool)> {
        struct Iter {
            raw_mask: u32,
            index: u8,
        }
        
        impl Iterator for Iter {
            type Item = (ItemKind, bool);

            fn next(&mut self) -> Option<Self::Item> {
                if self.index > ItemKind::LAST as u8 {
                    None
                } else {
                    let contained = (self.raw_mask >> self.index as u32) & 1 == 1;
                    let item = unsafe { transmute(self.index) };
                    self.index += 1;
                    Some((item, contained))
                }
            }
        }
        
        Iter {
            raw_mask: self.0,
            index: 0,
        }
    }
}

pub struct EnchantmentMask(u64);

impl EnchantmentMask {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn from_slice(slice: &[EnchantmentKind]) -> Self {
        let mut mask = Self::empty();
        let mut index = 0;

        while index < slice.len() {
            mask = mask.add(slice[index]);
            index += 1;
        }

        mask
    }

    #[inline]
    pub const fn add(self, kind: EnchantmentKind) -> Self {
        Self(self.0 | (1 << kind as u64))
    }

    #[inline]
    pub const fn has(self, kind: EnchantmentKind) -> bool {
        self.0 & (1 << kind as u64) != 0
    }

    #[inline]
    pub const fn remove(self, kind: EnchantmentKind) -> Self {
        Self(self.0 & !(1 << kind as u64))
    }
    
    pub fn iter(self) -> impl Iterator<Item = (EnchantmentKind, bool)> {
        struct Iter {
            raw_mask: u64,
            index: u8,
        }
        
        impl Iterator for Iter {
            type Item = (EnchantmentKind, bool);

            fn next(&mut self) -> Option<Self::Item> {
                if self.index > EnchantmentKind::LAST as u8 {
                    None
                } else {
                    let contained = (self.raw_mask >> self.index as u32) & 1 == 1;
                    let item = unsafe { transmute(self.index) };
                    self.index += 1;
                    Some((item, contained))
                }
            }
        }
        
        Iter {
            raw_mask: self.0,
            index: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Enchantments(u128);

impl Debug for Enchantments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().filter(|e| e.level() > 0)).finish()
    }
}

impl Enchantments {
    #[inline]
    pub const fn empty() -> Self {
        Enchantments(0)
    }

    #[inline]
    pub fn set(self, enchantment: Enchantment) -> Self {
        let factor = enchantment.kind() as u128 * 3;
        Self((self.0 & !(7 << factor)) | ((enchantment.level() as u128) << factor))
    }

    #[inline]
    pub const fn get_level(self, kind: EnchantmentKind) -> u8 {
        ((self.0 >> (kind as u128 * 3)) & 7) as u8
    }

    #[inline]
    pub const fn get_enchantment(self, kind: EnchantmentKind) -> Enchantment {
        Enchantment::new(kind, self.get_level(kind)).unwrap()
    }

    #[inline]
    pub const fn has(self, kind: EnchantmentKind) -> bool {
        self.0 & (7 << (kind as u128 * 3)) != 0
    }
    
    #[inline]
    pub const fn size(self) -> u8 {
        let mut count = 0_u8;
        let mut i = 0;
        
        while i < 42 {
            if (self.0 >> (i * 3)) & 7 != 0 {
                count += 1;
            }
            
            i += 1;
        }
        
        count
    }
    
    #[inline]
    pub const fn raw(self) -> u128 {
        self.0
    }

    #[inline]
    pub fn iter_contained(self) -> impl Iterator<Item = Enchantment> {
        self.iter().filter(|e| e.level() > 0)
    }
    
    pub fn iter(self) -> impl Iterator<Item = Enchantment> {
        struct Iter {
            e: Enchantments,
            index: u8,
        }

        impl Iterator for Iter {
            type Item = Enchantment;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index > EnchantmentKind::LAST as u8 {
                    None
                } else {
                    let kind = unsafe { transmute(self.index) };
                    let level = self.e.get_level(kind);
                    self.index += 1;
                    unsafe { Some(Enchantment::new(kind, level).unwrap_unchecked()) }
                }
            }
        }

        Iter { e: self, index: 0 }
    }
}

impl Into<EnchantmentMask> for Enchantments {
    fn into(self) -> EnchantmentMask {
        self.iter()
            .filter_map(|e| (e.level() > 0).then_some(e.kind()))
            .fold(EnchantmentMask::empty(), |mask, kind| mask.add(kind))
    }
}

#[cfg(test)]
mod tests {
    mod item_mask {
        use crate::bits::ItemMask;
        use crate::items::ItemKind;

        #[test]
        fn from_slice() {
            assert_eq!(
                ItemMask::from_slice(&[
                    ItemKind::Sword,
                    ItemKind::Shield,
                    ItemKind::Axe
                ]),
                ItemMask::empty()
                    .add(ItemKind::Sword)
                    .add(ItemKind::Shield)
                    .add(ItemKind::Axe)
            );
        }
        
        #[test]
        fn add() {
            assert_eq!(
                ItemMask::empty()
                    .add(ItemKind::Sword)
                    .add(ItemKind::Boots)
                    .0,
                (1 << ItemKind::Sword as u32) | (1 << ItemKind::Boots as u32)
            );
        }
        
        #[test]
        fn remove() {
            assert_eq!(ItemMask::empty().remove(ItemKind::Sword), ItemMask::empty());
            
            assert_eq!(
                ItemMask::empty()
                    .add(ItemKind::Boots)
                    .add(ItemKind::Compass)
                    .remove(ItemKind::Boots),
                ItemMask::empty()
                    .add(ItemKind::Compass)
            );
        }
    }
}