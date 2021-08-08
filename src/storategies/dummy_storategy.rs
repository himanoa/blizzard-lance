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
    fn execute(&self, _ctx: &ActionStrategyContext) -> Result<()> {
        Ok(())
    }
}

impl PreemptiveStrategy for DummyStrategy {
    fn execute(&self, _ctx: &PreemptiveStrategyContext) -> Result<usize> {
        Ok(12)
    }
}

impl BattleStrategy for DummyStrategy {}

