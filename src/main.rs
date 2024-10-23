use std::collections::HashMap;
use crate::anvil::{best_branch, Enchantment, EnchantmentKind, generate_branches, Item, ItemKind};

mod anvil;
mod dist;
mod anvil2;

fn main() {
    let mut item_pool = Vec::new();
    let mut combine_cache = HashMap::new();

    item_pool.push(Item {
        enchantments: vec![],
        prior_work_penalty: 0,
        kind: ItemKind::Axe,
    });

    item_pool.push(Item {
        enchantments: vec![
            Enchantment {
                kind: EnchantmentKind::Mending,
                level: 1,
            }
        ],
        prior_work_penalty: 0,
        kind: ItemKind::Book,
    });

    item_pool.push(Item {
        enchantments: vec![
            Enchantment {
                kind: EnchantmentKind::Efficiency,
                level: 5,
            }
        ],
        prior_work_penalty: 0,
        kind: ItemKind::Book,
    });

    item_pool.push(Item {
        enchantments: vec![
            Enchantment {
                kind: EnchantmentKind::SilkTouch,
                level: 1,
            }
        ],
        prior_work_penalty: 0,
        kind: ItemKind::Book,
    });

    item_pool.push(Item {
        enchantments: vec![
            Enchantment {
                kind: EnchantmentKind::Sharpness,
                level: 5,
            }
        ],
        prior_work_penalty: 0,
        kind: ItemKind::Book,
    });

    item_pool.push(Item {
        enchantments: vec![
            Enchantment {
                kind: EnchantmentKind::Unbreaking,
                level: 3,
            }
        ],
        prior_work_penalty: 0,
        kind: ItemKind::Book,
    });

    let branches = generate_branches(&[0, 1, 2, 3, 4, 5], &mut item_pool, &mut combine_cache).unwrap();
    println!("{}", best_branch(&branches).unwrap().display(&item_pool));
}