use crate::battle::{BattleActor, BattleArea, BattleField};
use crate::ryodansekai::Skill;

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

pub fn reducer(prev_battle_field: &BattleField, action: &ActorAction) -> BattleField {
    todo!()
}

pub trait BattleFieldMutation {
    fn dispatch(&mut self, action: &ActorAction);
}

#[derive(Debug, PartialEq, Eq)]
pub struct BattleFieldMutationImpl<'a> {
    applied_actions: Vec<ActorAction>,
    current_state: &'a mut BattleField,
}

impl<'a> BattleFieldMutation for BattleFieldMutationImpl<'a> {
    fn dispatch(&mut self, action: &ActorAction){
        *self.current_state = reducer(&self.current_state, action);
        self.applied_actions.push(action.clone());
    }
}
