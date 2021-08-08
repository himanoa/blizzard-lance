use crate::{battle::BattleField, battle_action::{ActorAction, BattleFieldMutation}};

pub struct ActionStrategyContext<'a> {
    pub battle_field: &'a BattleField,
    pub mutation: Box<dyn BattleFieldMutation>,
}

pub struct PreemptiveStrategyContext<'a> {
    pub battle_field: &'a BattleField,
    pub mutation: Box<dyn BattleFieldMutation>,
}

pub trait ActionStrategy {
    fn execute(&self, ctx: &ActionStrategyContext) -> Vec<ActorAction>;
}

pub trait PreemptiveStrategy {
    fn execute(&self, ctx: &PreemptiveStrategyContext) -> Vec<ActorAction>;
}

pub trait BattleStrategy
{
    fn action_stragegy() -> Box<dyn ActionStrategy>;
    fn preemptive_strategy() -> Box<dyn PreemptiveStrategy>;
}
