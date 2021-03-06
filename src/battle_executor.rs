use std::cell::RefCell;
use std::rc::Rc;

use anyhow::anyhow;

use crate::battle::{BattleActor, BattleField};
use crate::battle_action::BattleFieldMutationImpl;
use crate::error::ApplicationError;
use crate::storategies::Strategies;
use crate::strategy::{PreemptiveStrategy, PreemptiveStrategyContext};
use crate::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct BattleExecutor {
    pub battle_field: Rc<RefCell<BattleField>>,
    pub current_turn: usize,
}

impl BattleExecutor {
    pub fn run(&mut self) -> Result<()> {
        let sorted_actors = self.execute_preemptive_phase();
        unimplemented!()
    }

    pub fn execute_preemptive_phase(&self) -> Vec<(BattleActor, usize)> {
        let mut result: Vec<(BattleActor, usize)> = self.battle_field.borrow()
            .available_actors()
            .map(move |actor| match &actor.strategy {
                Strategies::DummyStrategy(strategy) => {
                    let mutation = BattleFieldMutationImpl::new(self.battle_field.clone());

                    let ctx = PreemptiveStrategyContext {
                        battle_field: self.battle_field.clone(),
                        mutation: Box::new(mutation),
                    };
                    let value = strategy.execute(&ctx)?;
                    Ok((actor.clone(), value))
                }
                _ => Err(anyhow!(ApplicationError::PreemtiveStrategyError {
                    msg: "unsupported strategy".to_string()
                })),
            }).flatten().collect();

        result.sort_by(|(_, a_value), (_, b_value)| {
            b_value.cmp(&a_value)
        });

        result
    }
}
