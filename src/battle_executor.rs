use crate::battle::BattleField;
use crate::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
struct BattleExecutor {
    pub battle_field: BattleField,
    pub current_turn: usize
}

impl BattleExecutor {
    fn run(&mut self) -> Result<()> {
        Ok(())   
    }
}
