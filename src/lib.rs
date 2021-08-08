pub mod battle;
pub mod battle_action;
pub mod equipment;
pub mod error;
pub mod ryodansekai;
pub mod strategy;
pub mod storategies;
pub mod weapon;
pub mod battle_executor;

pub type Result<T> = std::result::Result<T, error::ApplicationError>;
