use crate::equipment::{Equipment, OffensiveEquipment};
use crate::ryodansekai::{
    Abillity, DamageExpression, DamageExpressionContext, HitDetectionArgument,
    HitDetectionExpression,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct LightWeightWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MediumWeightWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct HeavyWeightWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PhysicalStrengthShootingWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MechanicalShootingWeapon {}

impl Equipment for LightWeightWeapon {
    fn apply(&self, abillity: Abillity) -> Abillity {
        abillity
    }
}

impl OffensiveEquipment for LightWeightWeapon {
    fn deal_damage_expression(&self) -> Box<DamageExpression> {
        Box::new(|abillity: Abillity, _| (abillity.py_str / 2 + 1) as usize)
    }
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression> {
        Box::new(|detection: HitDetectionArgument| {
            let value = detection.abillity.dex + detection.dice_result as isize;
            if value < 0 {
                return 0;
            }
            value as usize
        })
    }
    fn decision_count(&self) -> usize {
        3
    }
}

impl Equipment for MediumWeightWeapon {
    fn apply(&self, abillity: Abillity) -> Abillity {
        abillity
    }
}

impl OffensiveEquipment for MediumWeightWeapon {
    fn deal_damage_expression(&self) -> Box<DamageExpression> {
        Box::new(|abillity: Abillity, _| {
            let value = abillity.py_str + 1;
            if value < 0 {
                return 0;
            }
            value as usize
        })
    }
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression> {
        Box::new(|hit_detaction: HitDetectionArgument| {
            let value = hit_detaction.dice_result as isize + hit_detaction.abillity.dex;
            if value < 0 {
                return 0;
            }
            value as usize
        })
    }
    fn decision_count(&self) -> usize {
        2
    }
}

impl Equipment for HeavyWeightWeapon {
    fn apply(&self, abillity: Abillity) -> Abillity {
        abillity
    }
}

impl OffensiveEquipment for HeavyWeightWeapon {
    fn deal_damage_expression(&self) -> Box<DamageExpression> {
        Box::new(|abillity: Abillity, _| {
            let value = abillity.py_str * 2 + 3;
            if value < 0 {
                return 0;
            }
            value as usize
        })
    }
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression> {
        Box::new(|hit_detaction: HitDetectionArgument| {
            let value = hit_detaction.dice_result as isize + hit_detaction.abillity.dex;
            if value < 0 {
                return 0;
            }
            value as usize
        })
    }
    fn decision_count(&self) -> usize {
        1
    }
}

impl Equipment for PhysicalStrengthShootingWeapon {
    fn apply(&self, abillity: Abillity) -> Abillity {
        abillity
    }
}

// NO TEST
impl OffensiveEquipment for PhysicalStrengthShootingWeapon {
    fn deal_damage_expression(&self) -> Box<DamageExpression> {
        Box::new(|abillity: Abillity, _| {
            let value = abillity.py_str as isize * 2 + 1;
            if value < 0 {
                return 0;
            }
            value as usize
        })
    }
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression> {
        Box::new(|hit_detaction: HitDetectionArgument| {
            hit_detaction.dice_result + hit_detaction.abillity.dex as usize
        })
    }
    fn decision_count(&self) -> usize {
        1
    }
}

impl<'a> Equipment for MechanicalShootingWeapon {
    fn apply(&self, abillity: Abillity) -> Abillity {
        abillity
    }
}

// NO TEST
impl<'a> OffensiveEquipment for MechanicalShootingWeapon {
    fn deal_damage_expression(&self) -> Box<DamageExpression> {
        Box::new(|_: Abillity, ctx: DamageExpressionContext| {
            let hit_detection = ctx
                .hit_detection_diff
                .expect("機械射撃武器なのに命中判定の差が存在しません");
            5 + hit_detection
        })
    }
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression> {
        Box::new(|hit_detaction: HitDetectionArgument| {
            hit_detaction.dice_result + hit_detaction.abillity.dex as usize
        })
    }
    fn decision_count(&self) -> usize {
        1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Weapon {
    LightWeightWeapon(LightWeightWeapon),
    MediumWeightWeapon(MediumWeightWeapon),
    HeavyWeightWeapon(HeavyWeightWeapon),
    PhysicalStrengthShootingWeapon(PhysicalStrengthShootingWeapon),
    MechanicalShootingWeapon(MechanicalShootingWeapon)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_light_weight_weapon_deal_damage_expression() {
        let equipment: LightWeightWeapon = Default::default();
        let abillity: Abillity = Default::default();
        assert_eq!(
            equipment.deal_damage_expression()(abillity, Default::default()),
            1
        )
    }

    #[test]
    fn test_light_weight_weapon_hit_detection_expression() {
        let equipment: LightWeightWeapon = Default::default();

        assert_eq!(equipment.hit_detection_expression()(Default::default()), 7);
        assert_eq!(
            equipment.hit_detection_expression()(HitDetectionArgument {
                abillity: Abillity {
                    dex: -3,
                    ..Default::default()
                },
                dice_result: 1
            }),
            0
        )
    }

    #[test]
    fn test_light_weight_weapon_decision_count() {
        let equipment: LightWeightWeapon = Default::default();

        assert_eq!(equipment.decision_count(), 3)
    }

    #[test]
    fn test_medium_weight_weapon_deal_damage_expression() {
        let equipment: MediumWeightWeapon = Default::default();
        let abillity: Abillity = Default::default();
        assert_eq!(
            equipment.deal_damage_expression()(abillity, Default::default()),
            2
        )
    }

    #[test]
    fn test_medium_weight_weapon_hit_detection_expression() {
        let equipment: MediumWeightWeapon = Default::default();

        assert_eq!(equipment.hit_detection_expression()(Default::default()), 7);
        assert_eq!(
            equipment.hit_detection_expression()(HitDetectionArgument {
                abillity: Abillity {
                    dex: -3,
                    ..Default::default()
                },
                dice_result: 1
            }),
            0
        )
    }

    #[test]
    fn test_medium_weight_weapon_decision_count() {
        let equipment: MediumWeightWeapon = Default::default();

        assert_eq!(equipment.decision_count(), 2)
    }

    #[test]
    fn test_heavy_weight_weapon_deal_damage_expression() {
        let equipment: HeavyWeightWeapon = Default::default();
        let abillity: Abillity = Default::default();
        assert_eq!(
            equipment.deal_damage_expression()(abillity, Default::default()),
            5
        )
    }

    #[test]
    fn test_heavy_weight_weapon_hit_detection_expression() {
        let equipment: HeavyWeightWeapon = Default::default();

        assert_eq!(equipment.hit_detection_expression()(Default::default()), 7);
        assert_eq!(
            equipment.hit_detection_expression()(HitDetectionArgument {
                abillity: Abillity {
                    dex: -3,
                    ..Default::default()
                },
                dice_result: 1
            }),
            0
        )
    }

    #[test]
    fn test_heavy_weight_weapon_decision_count() {
        let equipment: HeavyWeightWeapon = Default::default();

        assert_eq!(equipment.decision_count(), 1)
    }
}
