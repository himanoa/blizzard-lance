use crate::equipment::{Equipment, OffensiveEquipment};
use crate::ryodansekai::{
    Abillity, DamageExpression, HitDetectionArgument, HitDetectionExpression,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct LightWeightWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MediumWeightWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct HeavyWeightWeapon {}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PhysicalStrengthShootingWeapon {}

impl Equipment for LightWeightWeapon {
    fn apply(&self, abillity: Abillity) -> Abillity {
        abillity
    }
}

impl OffensiveEquipment for LightWeightWeapon {
    fn deal_damage_expression(&self) -> Box<DamageExpression> {
        return Box::new(|abillity: Abillity| (abillity.py_str / 2 + 1) as usize);
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
        Box::new(|abillity: Abillity| {
            let value = abillity.py_str + 1;
            if value < 0 {
                return 0;
            }
            return value as usize;
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
        Box::new(|abillity: Abillity| {
            let value = abillity.py_str * 2 + 3;
            if value < 0 {
                return 0;
            }
            return value as usize;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_light_weight_weapon_deal_damage_expression() {
        let equipment: LightWeightWeapon = Default::default();
        let abillity: Abillity = Default::default();
        assert_eq!(equipment.deal_damage_expression()(abillity), 1)
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
        assert_eq!(equipment.deal_damage_expression()(abillity), 2)
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
        assert_eq!(equipment.deal_damage_expression()(abillity), 5)
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
