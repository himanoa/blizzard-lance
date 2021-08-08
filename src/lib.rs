pub mod battle;
pub mod battle_action;
pub mod battle_executor;
pub mod equipment;
pub mod error;
pub mod ryodansekai;
pub mod storategies;
pub mod strategy;
pub mod weapon;

pub type Result<T> = anyhow::Result<T>;
