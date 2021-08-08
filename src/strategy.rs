use crate::{battle::BattleField, battle_action::BattleFieldMutation, Result};

pub struct ActionStrategyContext<'a> {
    pub battle_field: &'a BattleField,
    pub mutation: Box<dyn BattleFieldMutation>,
}

pub struct PreemptiveStrategyContext<'a> {
    pub battle_field: &'a BattleField,
    pub mutation: Box<dyn BattleFieldMutation>,
}

pub trait ActionStrategy {
    fn execute(&self, ctx: &ActionStrategyContext) -> Result<()>;
}

pub trait PreemptiveStrategy {
    fn execute(&self, ctx: &PreemptiveStrategyContext) -> Result<()>;
}

pub trait BattleStrategy {
    fn action_stragegy() -> Box<dyn ActionStrategy>;
    fn preemptive_strategy() -> Box<dyn PreemptiveStrategy>;
}
