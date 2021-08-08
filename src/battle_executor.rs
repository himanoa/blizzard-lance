use std::rc::Rc;
use std::cell::RefCell;

use anyhow::anyhow;

use crate::battle::{BattleField,BattleActor};
use crate::battle_action::BattleFieldMutationImpl;
use crate::strategy::{PreemptiveStrategy, PreemptiveStrategyContext};
use crate::storategies::Strategies;
use crate::Result;
use crate::error::ApplicationError;

#[derive(Debug, PartialEq, Eq)]
pub struct BattleExecutor {
    pub battle_field: Rc<RefCell<BattleField>>,
    pub current_turn: usize
}

impl BattleExecutor {
    pub fn run(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn execute_preemptive_phase(&self) -> Result<impl Iterator<Item=&BattleActor>> {
        let field = self.battle_field.clone();
        let ctx = Rc::new(PreemptiveStrategyContext {
            battle_field: field.as_ref(),
            mutation: Rc::new(
                BattleFieldMutationImpl::new(
                    unimplemented!()
                )
            )
        });
        let field = self.battle_field.clone();
        let results: Vec<(&BattleActor, usize)> = field.into_inner().available_actors().map(|actor| {
            match &actor.strategy {
                Strategies::DummyStrategy(strategy) => {
                    let value = strategy.execute(&ctx)?;
                    Ok((actor, value))
                },
                _ => Err(anyhow!(ApplicationError::PreemtiveStrategyError{msg: "unsupported strategy".to_string()}))
            }
        }).flatten().collect();
        results.sort_by(|(_, a_value), (_, b_value)| { b_value.cmp(&a_value) });
        Ok(results.iter().map(|(actor, _)| *actor))
    }
}
