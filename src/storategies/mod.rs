use self::dummy_storategy::DummyStrategy;

pub mod dummy_storategy;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Strategies {
    DummyStrategy(DummyStrategy)
}

impl Default for Strategies {
    fn default() -> Self {
        Strategies::DummyStrategy(DummyStrategy {})
    }
}
