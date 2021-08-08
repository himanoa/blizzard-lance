use crate::{ryodansekai::Actor, storategies::Strategies};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleActor {
    current_position: BattleArea,
    actor: Actor,
    strategy: Strategies,
}

impl Default for BattleActor {
    fn default() -> Self {
        Self {
            current_position: BattleArea::PlayerRearGuard,
            actor: Default::default(),
            strategy: Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleArea {
    PlayerRearGuard,
    MiddleGuard,
    EnemyRearGuard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleField {
    participating_actors: Vec<BattleActor>,
    current_turn: usize,
}

impl BattleField {
    pub fn new(participating_actors: Vec<BattleActor>) -> Self {
        Self {
            participating_actors,
            current_turn: 1,
        }
    }

    #[allow(dead_code)]
    fn player_rear_guard_actors(&self) -> impl Iterator<Item = &BattleActor> {
        self.participating_actors
            .iter()
            .filter(|participating_actor| {
                participating_actor.current_position == BattleArea::PlayerRearGuard
            })
    }

    #[allow(dead_code)]
    fn middle_guard_actors(&self) -> impl Iterator<Item = &BattleActor> {
        self.participating_actors
            .iter()
            .filter(|participating_actor| {
                participating_actor.current_position == BattleArea::MiddleGuard
            })
    }

    #[allow(dead_code)]
    fn enemy_rear_actors(&self) -> impl Iterator<Item = &BattleActor> {
        self.participating_actors
            .iter()
            .filter(|participating_actor| {
                participating_actor.current_position == BattleArea::EnemyRearGuard
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{BattleActor, BattleArea, BattleField};

    impl Default for BattleField {
        fn default() -> Self {
            BattleField {
                participating_actors: vec![
                    BattleActor {
                        current_position: BattleArea::PlayerRearGuard,
                        ..Default::default()
                    },
                    BattleActor {
                        current_position: BattleArea::MiddleGuard,
                        ..Default::default()
                    },
                    BattleActor {
                        current_position: BattleArea::EnemyRearGuard,
                        ..Default::default()
                    },
                ],
                current_turn: 1,
            }
        }
    }

    #[test]
    fn test_player_rear_guard_actors() {
        let battle_field: BattleField = Default::default();
        let actual_actors: Vec<BattleActor> =
            battle_field.player_rear_guard_actors().cloned().collect();
        assert_eq!(
            actual_actors,
            vec![battle_field.participating_actors[0].clone()]
        )
    }

    #[test]
    fn test_middle_guard_actors() {
        let battle_field: BattleField = Default::default();
        let actual_actors: Vec<BattleActor> = battle_field.middle_guard_actors().cloned().collect();
        assert_eq!(
            actual_actors,
            vec![battle_field.participating_actors[1].clone()]
        )
    }

    #[test]
    fn test_enemy_rear_guard_actors() {
        let battle_field: BattleField = Default::default();
        let actual_actors: Vec<BattleActor> = battle_field.enemy_rear_actors().cloned().collect();
        assert_eq!(
            actual_actors,
            vec![battle_field.participating_actors[2].clone()]
        )
    }
}
