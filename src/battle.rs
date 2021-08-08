use crate::ryodansekai::Actor;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BattleArea {
    player_rear_guard: Vec<Actor>,
    middle_guard: Vec<Actor>,
    enemy_rear_guard: Vec<Actor>
}

struct BattleField {
    area: BattleArea,
    current_turn: usize,

}
