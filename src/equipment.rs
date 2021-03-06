use crate::ryodansekai::{Abillity, DamageExpression, EvadeExpression, HitDetectionExpression};

pub trait Equipment {
    fn apply(&self, abillity: Abillity) -> Abillity;
}

pub trait OffensiveEquipment {
    fn deal_damage_expression(&self) -> Box<DamageExpression>;
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression>;
    fn decision_count(&self) -> usize;
}

pub trait DefensiveEquipment {
    fn take_damage_expression(&self) -> Box<DamageExpression>;
    fn evade_expression(&self) -> Box<EvadeExpression>;
}
