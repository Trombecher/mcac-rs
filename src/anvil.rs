use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use arrayvec::ArrayVec;
use crate::bits::Enchantments;
use crate::dist::DIST;
use crate::enchantments::Enchantment;
use crate::items::ItemKind;
use crate::dist::MAX_ITEMS;

#[derive(Debug)]
pub enum CombinationError {
    IncompatibleItems {
        target: ItemKind,
        sacrifice: ItemKind,
    }
}

#[derive(Debug, Clone)]
pub struct Step {
    pub target: Item,
    pub sacrifice: Item,
    pub result: Item,
    pub cost: u16,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Item {
    pub enchantments: Enchantments,
    pub prior_work_penalty: u16,
    pub kind: ItemKind,
}

impl Item {
    pub fn combine(target: Item, sacrifice: Item) -> Result<Step, CombinationError> {
        if target.kind != sacrifice.kind && sacrifice.kind != ItemKind::Book {
            return Err(CombinationError::IncompatibleItems {
                target: target.kind,
                sacrifice: target.kind,
            });
        }
        
        let mut step = Step {
            target,
            sacrifice,
            result: Item {
                enchantments: target.enchantments,
                prior_work_penalty: target.prior_work_penalty.max(sacrifice.prior_work_penalty) * 2 + 1,
                kind: target.kind,
            },
            cost: target.prior_work_penalty + sacrifice.prior_work_penalty,
        };

        'se: for sacrifice_enchantment in sacrifice.enchantments.iter_contained() {
            if !sacrifice_enchantment.kind().applicable_to().has(target.kind) {
                continue;
            }
            
            let mut target_enchantment_match_level = None;
            
            for target_enchantment in target.enchantments.iter_contained() {
                if sacrifice_enchantment.kind().incompatible_with().has(target_enchantment.kind()) {
                    step.cost += 1;
                    break 'se;
                }
                
                if target_enchantment.kind() == sacrifice_enchantment.kind() {
                    target_enchantment_match_level = Some(target_enchantment.level());
                    break;
                }
            }
            
            if let Some(mut result_enchantment_level) = target_enchantment_match_level {
                if result_enchantment_level < sacrifice_enchantment.level() {
                    result_enchantment_level = sacrifice_enchantment.level();
                } else if result_enchantment_level == sacrifice_enchantment.level()
                    && result_enchantment_level < sacrifice_enchantment.kind().max_level() {
                    result_enchantment_level += 1;
                }

                step.cost += (if sacrifice.kind == ItemKind::Book {
                    sacrifice_enchantment.kind().book_multiplier()
                } else {
                    sacrifice_enchantment.kind().item_multiplier()
                } * result_enchantment_level) as u16;

                step.result.enchantments = step.result.enchantments.set(Enchantment::new(
                    sacrifice_enchantment.kind(),
                    result_enchantment_level
                ).unwrap());
            } else {
                // There is no match.

                step.cost += (if sacrifice.kind == ItemKind::Book {
                    sacrifice_enchantment.kind().book_multiplier()
                } else {
                    sacrifice_enchantment.kind().item_multiplier()
                } * sacrifice_enchantment.level()) as u16;

                step.result.enchantments = step.result.enchantments.set(sacrifice_enchantment);
            }
        }
        
        Ok(step)
    }
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub steps: ArrayVec<Step, MAX_ITEMS>,
    pub total_cost: u16,
}

impl Branch {
    pub fn of_two(first: Item, second: Item) -> Result<Self, CombinationError> {
        let first_with_second = Item::combine(first, second);
        let second_with_first = Item::combine(second, first);

        let best_step = if let Ok(first) = first_with_second {
            if let Ok(second) = second_with_first {
                if first.cost < second.cost {
                    first
                } else {
                    second
                }
            } else {
                first
            }
        } else {
            second_with_first?
        };

        Ok(Branch {
            total_cost: best_step.cost,
            steps: {
                let mut steps = ArrayVec::new_const();
                steps.push(best_step);
                steps
            },
        })
    }
}

pub fn branch_iterator<'a>(items: &'a [Item]) -> impl Coroutine<Yield = Branch, Return = Result<(), CombinationError>> + 'a + Unpin {
    Box::pin(#[coroutine] static || {
        if items.len() == 1 {
            yield Branch {
                steps: ArrayVec::new_const(),
                total_cost: 0,
            };
            return Ok(());
        }
        
        if items.len() == 2 {
            yield Branch::of_two(items[0], items[1])?;
            return Ok(());
        }

        for (left, right) in DIST[items.len() - 3].iter().copied() {
            let mut left_items = ArrayVec::<_, MAX_ITEMS>::new_const();
            for index in left.iter().copied() {
                left_items.push(items[index as usize]);
            }
            
            let left_has_one_item = left.len() == 1;
            let mut left_branches_iter = branch_iterator(left_items.as_slice());
            
            let mut right_items = ArrayVec::<_, MAX_ITEMS>::new_const();
            for index in right.iter().copied() {
                right_items.push(items[index as usize]);
            }
            
            loop {
                match Pin::new(&mut left_branches_iter).resume(()) {
                    CoroutineState::Yielded(left_branch) => {
                        let mut right_branches_iter = branch_iterator(right_items.as_slice());
                        
                        loop {
                            match Pin::new(&mut right_branches_iter).resume(()) {
                                CoroutineState::Yielded(right_branch) => {
                                    let first_item = if left_has_one_item {
                                        left_items[0]
                                    } else {
                                        left_branch.steps.last().unwrap().result
                                    };
                                    
                                    let second_item = right_branch.steps.last().unwrap().result;
                                    
                                    let mut new_branch = Branch::of_two(first_item, second_item)?;
                                    
                                    let last_step = new_branch.steps.pop().unwrap();
                                    new_branch.steps.extend(left_branch.steps.iter().cloned());
                                    new_branch.steps.extend(right_branch.steps.iter().cloned());
                                    new_branch.steps.push(last_step);
                                    new_branch.total_cost += left_branch.total_cost + right_branch.total_cost;
                                    
                                    yield new_branch;
                                }
                                CoroutineState::Complete(x) => break x?
                            }
                        }
                    }
                    CoroutineState::Complete(x) => break x?
                }
            }
        }
        
        Ok(())
    })
}