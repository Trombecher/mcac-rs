use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::dist::DIST;

pub const MAX_SUPPORTED_ITEMS: usize = 10;

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
    pub fn max_level(self) -> u8 {
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

    pub fn applicable_to_book_and(self) -> &'static [ItemKind] {
        match self {
            EnchantmentKind::Mending => &[
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
            ],
            EnchantmentKind::Unbreaking => &[
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
            ],
            EnchantmentKind::CurseOfBinding => &[
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
                ItemKind::Elytra,
                ItemKind::CarvedPumpkin,
                ItemKind::Head
            ],
            EnchantmentKind::CurseOfVanishing => &[
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
            ],
            EnchantmentKind::Protection => &[
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ],
            EnchantmentKind::BlastProtection => &[
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ],
            EnchantmentKind::FireProtection => &[
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ],
            EnchantmentKind::ProjectileProtection => &[
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots
            ],
            EnchantmentKind::Thorns => &[
                ItemKind::Helmet,
                ItemKind::Chestplate,
                ItemKind::Leggings,
                ItemKind::Boots,
            ],
            EnchantmentKind::Respiration => &[
                ItemKind::Helmet,
            ],
            EnchantmentKind::AquaAffinity => &[
                ItemKind::Helmet
            ],
            EnchantmentKind::SwiftSneak => &[
                ItemKind::Leggings
            ],
            EnchantmentKind::DepthStrider => &[
                ItemKind::Boots
            ],
            EnchantmentKind::FrostWalker => &[
                ItemKind::Boots
            ],
            EnchantmentKind::FeatherFalling => &[
                ItemKind::Boots
            ],
            EnchantmentKind::SoulSpeed => &[
                ItemKind::Boots
            ],
            EnchantmentKind::SweepingEdge => &[
                ItemKind::Axe,
            ],
            EnchantmentKind::Sharpness => &[
                ItemKind::Sword,
                ItemKind::Axe,
            ],
            EnchantmentKind::Smite => &[
                ItemKind::Sword,
                ItemKind::Axe,
            ],
            EnchantmentKind::BaneOfArthropods => &[
                ItemKind::Sword,
                ItemKind::Axe,
            ],
            EnchantmentKind::Knockback => &[
                ItemKind::Sword
            ],
            EnchantmentKind::FireAspect => &[
                ItemKind::Sword,
            ],
            EnchantmentKind::Looting => &[
                ItemKind::Sword
            ],
            EnchantmentKind::SilkTouch => &[
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
            ],
            EnchantmentKind::Efficiency => &[
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
                ItemKind::Shears
            ],
            EnchantmentKind::Fortune => &[
                ItemKind::Pickaxe,
                ItemKind::Shovel,
                ItemKind::Axe,
                ItemKind::Hoe,
            ],
            EnchantmentKind::Power => &[
                ItemKind::Bow
            ],
            EnchantmentKind::Punch => &[
                ItemKind::Bow
            ],
            EnchantmentKind::Flame => &[
                ItemKind::Bow
            ],
            EnchantmentKind::Infinity => &[
                ItemKind::Bow
            ],
            EnchantmentKind::LuckOfTheSea => &[
                ItemKind::FishingRod
            ],
            EnchantmentKind::Lure => &[
                ItemKind::FishingRod
            ],
            EnchantmentKind::Impaling => &[
                ItemKind::Trident
            ],
            EnchantmentKind::Riptide => &[
                ItemKind::Trident
            ],
            EnchantmentKind::Loyalty => &[
                ItemKind::Trident
            ],
            EnchantmentKind::Channeling => &[
                ItemKind::Trident
            ],
            EnchantmentKind::Multishot => &[
                ItemKind::Crossbow
            ],
            EnchantmentKind::Piercing => &[
                ItemKind::Crossbow
            ],
            EnchantmentKind::QuickCharge => &[
                ItemKind::Crossbow
            ],
        }
    }

    pub fn incompatible_with(self) -> &'static [EnchantmentKind] {
        match self {
            EnchantmentKind::Mending => &[
                EnchantmentKind::Infinity
            ],
            EnchantmentKind::Unbreaking => &[],
            EnchantmentKind::CurseOfBinding => &[],
            EnchantmentKind::CurseOfVanishing => &[],
            EnchantmentKind::Protection => &[
                EnchantmentKind::FireProtection,
                EnchantmentKind::BlastProtection,
                EnchantmentKind::ProjectileProtection,
            ],
            EnchantmentKind::BlastProtection => &[
                EnchantmentKind::FireProtection,
                EnchantmentKind::Protection,
                EnchantmentKind::ProjectileProtection,
            ],
            EnchantmentKind::FireProtection => &[
                EnchantmentKind::Protection,
                EnchantmentKind::BlastProtection,
                EnchantmentKind::ProjectileProtection,
            ],
            EnchantmentKind::ProjectileProtection => &[
                EnchantmentKind::FireProtection,
                EnchantmentKind::BlastProtection,
                EnchantmentKind::Protection,
            ],
            EnchantmentKind::Thorns => &[],
            EnchantmentKind::Respiration => &[],
            EnchantmentKind::AquaAffinity => &[],
            EnchantmentKind::SwiftSneak => &[],
            EnchantmentKind::DepthStrider => &[
                EnchantmentKind::FrostWalker
            ],
            EnchantmentKind::FrostWalker => &[
                EnchantmentKind::DepthStrider
            ],
            EnchantmentKind::FeatherFalling => &[],
            EnchantmentKind::SoulSpeed => &[],
            EnchantmentKind::SweepingEdge => &[],
            EnchantmentKind::Sharpness => &[
                EnchantmentKind::Smite,
                EnchantmentKind::BaneOfArthropods,
            ],
            EnchantmentKind::Smite => &[
                EnchantmentKind::Sharpness,
                EnchantmentKind::BaneOfArthropods,
            ],
            EnchantmentKind::BaneOfArthropods => &[
                EnchantmentKind::Sharpness,
                EnchantmentKind::Smite,
            ],
            EnchantmentKind::Knockback => &[],
            EnchantmentKind::FireAspect => &[],
            EnchantmentKind::Looting => &[],
            EnchantmentKind::SilkTouch => &[
                EnchantmentKind::Fortune
            ],
            EnchantmentKind::Efficiency => &[],
            EnchantmentKind::Fortune => &[
                EnchantmentKind::SilkTouch
            ],
            EnchantmentKind::Power => &[],
            EnchantmentKind::Punch => &[],
            EnchantmentKind::Flame => &[],
            EnchantmentKind::Infinity => &[
                EnchantmentKind::Mending
            ],
            EnchantmentKind::LuckOfTheSea => &[],
            EnchantmentKind::Lure => &[],
            EnchantmentKind::Impaling => &[],
            EnchantmentKind::Riptide => &[
                EnchantmentKind::Loyalty,
                EnchantmentKind::Channeling
            ],
            EnchantmentKind::Loyalty => &[
                EnchantmentKind::Riptide
            ],
            EnchantmentKind::Channeling => &[
                EnchantmentKind::Riptide
            ],
            EnchantmentKind::Multishot => &[
                EnchantmentKind::Piercing
            ],
            EnchantmentKind::Piercing => &[
                EnchantmentKind::Multishot
            ],
            EnchantmentKind::QuickCharge => &[],
        }
    }

    pub fn item_multiplier(self) -> u8 {
        match self {
            EnchantmentKind::Mending => 4,
            EnchantmentKind::Unbreaking => 2,
            EnchantmentKind::CurseOfBinding => 8,
            EnchantmentKind::CurseOfVanishing => 8,
            EnchantmentKind::Protection => 1,
            EnchantmentKind::BlastProtection => 4,
            EnchantmentKind::FireProtection => 2,
            EnchantmentKind::ProjectileProtection => 2,
            EnchantmentKind::Thorns => 8,
            EnchantmentKind::Respiration => 4,
            EnchantmentKind::AquaAffinity => 4,
            EnchantmentKind::SwiftSneak => 8,
            EnchantmentKind::DepthStrider => 4,
            EnchantmentKind::FrostWalker => 4,
            EnchantmentKind::FeatherFalling => 2,
            EnchantmentKind::SoulSpeed => 8,
            EnchantmentKind::SweepingEdge => 4,
            EnchantmentKind::Sharpness => 1,
            EnchantmentKind::Smite => 2,
            EnchantmentKind::BaneOfArthropods => 2,
            EnchantmentKind::Knockback => 2,
            EnchantmentKind::FireAspect => 4,
            EnchantmentKind::Looting => 4,
            EnchantmentKind::SilkTouch => 8,
            EnchantmentKind::Efficiency => 1,
            EnchantmentKind::Fortune => 4,
            EnchantmentKind::Power => 1,
            EnchantmentKind::Punch => 4,
            EnchantmentKind::Flame => 4,
            EnchantmentKind::Infinity => 8,
            EnchantmentKind::LuckOfTheSea => 4,
            EnchantmentKind::Lure => 4,
            EnchantmentKind::Impaling => 4,
            EnchantmentKind::Riptide => 4,
            EnchantmentKind::Loyalty => 1,
            EnchantmentKind::Channeling => 8,
            EnchantmentKind::Multishot => 4,
            EnchantmentKind::Piercing => 1,
            EnchantmentKind::QuickCharge => 2,
        }
    }

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

#[derive(Clone)]
pub struct Enchantment {
    pub kind: EnchantmentKind,
    pub level: u8,
}

impl Display for Enchantment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {}",
            self.kind,
            match self.level {
                1 => "I",
                2 => "II",
                3 => "III",
                4 => "IV",
                5 => "V",
                _ => panic!("TODO")
            }
        )
    }
}

pub struct Item {
    pub enchantments: Vec<Enchantment>,
    pub prior_work_penalty: u16,
    pub kind: ItemKind,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} with pwp {} with enchantments",
            self.kind,
            self.prior_work_penalty
        )?;

        for enchantment in self.enchantments.iter() {
            write!(
                f,
                " {}",
                enchantment
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum CombinationError {
    IncompatibleItems {
        target: ItemKind,
        sacrifice: ItemKind,
    }
}

#[derive(Clone)]
pub struct Step {
    target: usize,
    sacrifice: usize,
    result: usize,
    cost: u16,
}

impl Step {
    pub fn display<'a>(&'a self, item_pool: &'a Vec<Item>) -> DisplayStep {
        DisplayStep {
            step: self,
            item_pool,
        }
    }
}

pub struct DisplayStep<'a> {
    step: &'a Step,
    item_pool: &'a Vec<Item>,
}

impl<'a> Display for DisplayStep<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Step with cost {}:\n\t{}\n\t+\n\t{}\n\t=\n\t{}",
            self.step.cost,
            self.item_pool[self.step.target],
            self.item_pool[self.step.sacrifice],
            self.item_pool[self.step.result]
        )
    }
}

pub struct Branch {
    steps: Vec<Step>,
    total_cost: u16,
}

impl Branch {
    pub fn display<'a>(&'a self, item_pool: &'a Vec<Item>) -> DisplayBranch {
        DisplayBranch {
            item_pool,
            branch: self,
        }
    }
}

pub struct DisplayBranch<'a> {
    branch: &'a Branch,
    item_pool: &'a Vec<Item>,
}

impl<'a> Display for DisplayBranch<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Steps of branch with total cost {}:",
            self.branch.total_cost,
        )?;

        for step in self.branch.steps.iter() {
            write!(
                f,
                "\n{}",
                step.display(self.item_pool)
            )?;
        }

        Ok(())
    }
}

impl Item {
    pub fn combine(target: usize, sacrifice: usize, item_pool: &mut Vec<Item>, cache: &mut Cache) -> Result<Step, CombinationError> {
        let target_item = get_item(target, item_pool);
        let sacrifice_item = get_item(sacrifice, item_pool);

        if let Some(step_template) = cache.get_mut(&(target, sacrifice)){
            return Ok(Step {
                target,
                sacrifice,
                result: step_template.result,
                cost: step_template.cost
                    - get_item(step_template.target, item_pool).prior_work_penalty
                    - get_item(step_template.sacrifice, item_pool).prior_work_penalty
                    + target_item.prior_work_penalty
                    + sacrifice_item.prior_work_penalty,
            });
        }

        if target_item.kind != sacrifice_item.kind && sacrifice_item.kind != ItemKind::Book {
            return Err(CombinationError::IncompatibleItems {
                target: target_item.kind,
                sacrifice: target_item.kind,
            });
        }

        let mut step = Step {
            target,
            sacrifice,
            result: item_pool.len(),
            cost: target_item.prior_work_penalty + sacrifice_item.prior_work_penalty,
        };

        let mut result_enchantments = target_item.enchantments.clone();

        'se: for sacrifice_enchantment in sacrifice_item.enchantments.iter() {
            let mut target_enchantment_match = None;

            if !sacrifice_enchantment.kind.applicable_to_book_and().contains(&target_item.kind) && target_item.kind != ItemKind::Book {
                continue;
            }

            // search for a match in the target enchantments
            for target_enchantment in target_item.enchantments.iter() {
                if sacrifice_enchantment.kind.incompatible_with().contains(&target_enchantment.kind) {
                    step.cost += 1;
                    break 'se;
                }

                if target_enchantment.kind == sacrifice_enchantment.kind {
                    target_enchantment_match = Some(target_enchantment);
                    break;
                }
            }

            match target_enchantment_match {
                None => {
                    // if no match, take on the sacrifice_enchantment and add the cost of that

                    step.cost += (if sacrifice_item.kind == ItemKind::Book {
                        sacrifice_enchantment.kind.book_multiplier()
                    } else {
                        sacrifice_enchantment.kind.item_multiplier()
                    } * sacrifice_enchantment.level) as u16;

                    result_enchantments.push(sacrifice_enchantment.clone())
                }
                Some(target_enchantment_match) => {
                    // if there is a match, clone the target_enchantment_match and update it according to the sacrifice_enchantment

                    let mut result_enchantment = target_enchantment_match.clone();

                    if result_enchantment.level < sacrifice_enchantment.level {
                        result_enchantment.level = sacrifice_enchantment.level;
                    } else if result_enchantment.level == sacrifice_enchantment.level
                        && result_enchantment.level < result_enchantment.kind.max_level() {
                        result_enchantment.level += 1;
                    }

                    step.cost += (if sacrifice_item.kind == ItemKind::Book {
                        result_enchantment.kind.book_multiplier()
                    } else {
                        result_enchantment.kind.item_multiplier()
                    } * result_enchantment.level) as u16;

                    result_enchantments.push(result_enchantment);
                }
            };
        }

        item_pool.push(Item {
            enchantments: result_enchantments,
            prior_work_penalty: std::cmp::max(target_item.prior_work_penalty, sacrifice_item.prior_work_penalty) * 2 + 1,
            kind: target_item.kind,
        });

        cache.insert((target, sacrifice), step.clone());
        Ok(step)
    }
}

#[inline]
pub fn get_item(item_index: usize, item_pool: &Vec<Item>) -> &Item {
    item_pool.get(item_index).unwrap()
}

pub fn best_branch(branches: &Vec<Branch>) -> Option<&Branch> {
    let mut best_branch = None;
    for branch in branches.iter() {
        match best_branch {
            None => best_branch = Some(branch),
            Some(current_branch) => {
                if branch.total_cost < current_branch.total_cost {
                    best_branch = Some(branch);
                }
            }
         }
    }
    best_branch
}

fn branch_of_two(first: usize, second: usize, item_pool: &mut Vec<Item>, combine_cache: &mut Cache) -> Result<Branch, CombinationError> {
    let first_with_second = Item::combine(
        first,
        second,
        item_pool,
        combine_cache
    );

    let second_with_first = Item::combine(
        second,
        first,
        item_pool,
        combine_cache
    );

    let best_step = match first_with_second {
        Ok(first) => {
            match second_with_first {
                Ok(second) => if first.cost < second.cost { first } else { second }
                _ => first
            }
        }
        _ => second_with_first?
    };

    Ok(Branch {
        total_cost: best_step.cost,
        steps: vec![
            best_step
        ],
    })
}

type Cache = HashMap<(usize, usize), Step>;

#[allow(mutable_transmutes)]
pub fn generate_branches(items: &[usize], item_pool: &mut Vec<Item>, combine_cache: &mut Cache) -> Result<Vec<Branch>, CombinationError> {
    assert!(items.len() > 1);
    if items.len() == 2 {
        let first = *items.get(0).unwrap();
        let second = *items.get(1).unwrap();

        return Ok(vec![branch_of_two(first, second, item_pool, combine_cache)?]);
    }

    let mut branches = Vec::new();

    for (left, right) in DIST[items.len() - 3] {
        let mut left_items = [255; MAX_SUPPORTED_ITEMS];
        for i in 0..left.len() {
            left_items[i] = items[left[i] as usize];
        }

        let left_has_one_item = left.len() == 1;

        // generate_branches(...) cannot handle one item
        let left_branches = if left_has_one_item {
            vec![Branch {
                steps: vec![],
                total_cost: 0,
            }]
        } else {
            generate_branches(&left_items[0..left.len()], item_pool, combine_cache)?
        };

        let mut right_items = [255; MAX_SUPPORTED_ITEMS];
        for i in 0..right.len() {
            right_items[i] = items[right[i] as usize];
        }

        let right_branches = generate_branches(&right_items[0..right.len()], item_pool, combine_cache)?;

        for left_branch in left_branches.iter() {
            for right_branch in right_branches.iter() {
                let first_item = if left_has_one_item {
                    left_items[0]
                } else {
                    left_branch.steps.last().unwrap().result
                };

                let second_item = right_branch.steps.last().unwrap().result;

                let mut new_branch = branch_of_two(first_item, second_item, item_pool, combine_cache)?;

                let last_step = new_branch.steps.pop().unwrap();
                new_branch.steps.extend(left_branch.steps.iter().cloned());
                new_branch.steps.extend(right_branch.steps.iter().cloned());
                new_branch.steps.push(last_step);
                new_branch.total_cost += left_branch.total_cost + right_branch.total_cost;

                branches.push(new_branch);
            }
        }
    }

    Ok(branches)
}