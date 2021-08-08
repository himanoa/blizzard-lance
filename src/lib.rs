pub mod battle;
pub mod battle_action;
pub mod equipment;
pub mod error;
pub mod ryodansekai;
pub mod weapon;
pub mod strategy;

pub type Result<T> = std::result::Result<T, error::ApplicationError>;
