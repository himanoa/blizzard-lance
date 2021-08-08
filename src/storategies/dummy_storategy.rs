use crate::strategy::{
    ActionStrategy,
    BattleStrategy,
    PreemptiveStrategy,
    ActionStrategyContext,
    PreemptiveStrategyContext,
};
use crate::Result;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct DummyStrategy {}

impl ActionStrategy for DummyStrategy {
    fn execute(&self, ctx: &ActionStrategyContext) -> Result<()> {
        Ok(())
    }
}

impl PreemptiveStrategy for DummyStrategy {
    fn execute(&self, ctx: &PreemptiveStrategyContext) -> Result<()> {
        Ok(())
    }
}

impl BattleStrategy for DummyStrategy {
    fn action_stragegy(&self) -> Box<&dyn ActionStrategy> {
        Box::new(self)
    }

    fn preemptive_strategy(&self) -> Box<&dyn PreemptiveStrategy> {
        Box::new(self)
    }
}
