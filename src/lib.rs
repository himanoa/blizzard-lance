pub mod equipment;
pub mod error;
pub mod ryodansekai;
pub mod weapon;
pub mod battle;

pub type Result<T> = std::result::Result<T, error::ApplicationError>;
