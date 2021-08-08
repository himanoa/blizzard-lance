use crate::battle_action::ActorAction;

pub trait ActionStrategy {
    fn execute(&self) -> Vec<ActorAction>;
}

pub trait PreemptiveStrategy {
    fn execute(&self) -> Vec<ActorAction>;
}

pub trait BattleStrategy
{
    fn action_stragegy() -> Box<dyn ActionStrategy>;
    fn preemptive_strategy() -> Box<dyn PreemptiveStrategy>;
}
