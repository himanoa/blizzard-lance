use crate::{battle::BattleField, battle_action::BattleFieldMutation, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ActionStrategyContext<'a> {
    pub battle_field: &'a BattleField,
    pub mutation: Box<dyn BattleFieldMutation>,
}

pub struct PreemptiveStrategyContext {
    pub battle_field: Rc<RefCell<BattleField>>,
    pub mutation: Box<dyn BattleFieldMutation>,
}

pub trait ActionStrategy {
    fn execute(&self, ctx: &ActionStrategyContext) -> Result<()>;
}

pub trait PreemptiveStrategy {
    fn execute(&self, ctx: &PreemptiveStrategyContext) -> Result<usize>;
}

pub trait BattleStrategy: ActionStrategy + PreemptiveStrategy {}
