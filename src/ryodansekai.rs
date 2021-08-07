#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Abillity {
    pub py_str: isize,
    pub dex: isize,
    pub int: isize,
    pub con: isize,
    pub luck: isize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ActorId(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Actor {
    pub id: ActorId,
    pub abillity: Abillity,
    pub skills: Vec<Skill>,
}

type Damage = usize;

type HitDetectionTarget = usize;

type DamageExpression = dyn Fn(Abillity, usize) -> Damage;

type EvadeTargetValue = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HitDetectionArgument {
    abillity: Abillity,
    dice_result: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Evade {
    abillity: Abillity,
    dice_result: usize,
    evade_target_value: EvadeTargetValue
}

type HitDetectionExpression = dyn Fn(HitDetectionArgument) -> HitDetectionTarget;
type EvadeExpression = dyn Fn(Evade) -> bool;

pub trait Equipment {
    fn apply(&self, abillity: Abillity) -> Abillity;
    fn deal_damage_expression(&self) -> Box<DamageExpression>;
    fn take_damage_expression(&self) -> Box<DamageExpression>;
    fn hit_detection_expression(&self) -> Box<HitDetectionExpression>;
    fn evade_expression(&self) -> Box<EvadeExpression>;
}

// TODO: あとで英語に直す
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Feature {
    特異な種族,
    老練,
    天才,
    狂戦士,
    不幸好き,
    自己再生,
    ピンチに強い,
    異形
}

// TODO: あとで英語に直す
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Skill {
    強打,
    必中,
    毒付与,
    バックスタブ,
    フェイント,
    二刀流,
    ぶん回し,
    急所熟知,
    転倒攻撃,
    ガード,
    カウンター,
    見切り,
    緊急回避,
    かばう,
    身代わり,
    格闘術,
    軽中量習熟I,
    軽中量習熟II,
    重量習熟I,
    重量習熟II,
    盾装備,
    高速装填,
    狙撃p2,
    特殊射撃,
    特殊射撃II,
    特殊範囲攻撃,
    治療,
    大治療,
    不屈,
    連続行動,
    先制力p3,
    能力分析,
    瞬間使用,
    所持数p3,
    情報力p3,
    発見p3,
    鋭敏感覚p3,
    泥棒の技p3,
    冒険技能p3,
    権威p3,
    交渉術p3,
    トラップx3,
    支給品袋,
    従者_使役動物,
    動物会話,
    飛行,
    ひらめき,
    神の寵愛,
    妨害,
    励まし,
    支援,
    集中,
}
