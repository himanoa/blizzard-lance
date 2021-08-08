use crate::battle::{BattleActor, BattleArea, BattleField};
use crate::ryodansekai::Skill;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ActorAction {
    Move {
        to: BattleArea,
    },
    Attack {
        to: BattleActor,
        dice_result: usize,
    },
    UseSkill {
        skill: Skill,
        maintence_dice_result: usize,
    },
    UseItem {
        maintence_dice_result: usize,
    },
    UseBeforeWill(),
    UseAfterWill(),
    Wait(),
}

pub fn reducer(_prev_battle_field: &BattleField, _action: &ActorAction) -> BattleField {
    todo!()
}

pub trait BattleFieldMutation {
    fn dispatch(&mut self, action: &ActorAction);
}

#[derive(Debug, PartialEq, Eq)]
pub struct BattleFieldMutationImpl {
    applied_actions: Vec<ActorAction>,
    current_state: Rc<RefCell<BattleField>>,
}

impl BattleFieldMutationImpl {
    pub fn new(current_state: Rc<RefCell<BattleField>>) -> Self {
        BattleFieldMutationImpl {
            applied_actions: vec![],
            current_state,
        }
    }
}

impl BattleFieldMutation for BattleFieldMutationImpl {
    fn dispatch(&mut self, action: &ActorAction) {
        let mut a = self.current_state.borrow_mut();
        *a = reducer(a.borrow(), action);
        self.applied_actions.push(action.clone());
    }
}
