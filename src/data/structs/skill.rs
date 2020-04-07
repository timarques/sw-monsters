use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Effect {
    BlockBeneficialEffects,
    Bomb,
    Brand,
    ContinuousDamage,
    Counter,
    CriticalResist,
    DecreaseAttackBar,
    DecreaseAttack,
    DecreaseDefense,
    DecreaseSpeed,
    Defend,
    Endure,
    Freeze,
    Glancing,
    Immunity,
    IncreaseAttackBar,
    IncreaseAttack,
    IncreaseCriticalRate,
    IncreaseDefense,
    IncreaseSpeed,
    Invincibility,
    Oblivion,
    ProtectSoul,
    Provoke,
    Recovery,
    Reflect,
    Revenge,
    Shield,
    Silence,
    Sleep,
    Stun,
    Threat,
    Unrecoverable,
    Vampire
}

impl From<String> for Effect {

    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "block-beneficial-effects" => Effect::BlockBeneficialEffects,
            "bomb" => Effect::Bomb,
            "brand" => Effect::Brand,
            "continuous-damage" => Effect::ContinuousDamage,
            "counter" => Effect::Counter,
            "critical-resist" => Effect::CriticalResist,
            "decrease-attack" => Effect::DecreaseAttack,
            "defend" => Effect::Defend,
            "endure" => Effect::Endure,
            "freeze" => Effect::Freeze,
            "glancing" => Effect::Glancing,
            "immunity" => Effect::Immunity,
            "increase-attack-bar" => Effect::IncreaseAttackBar,
            "increase-attack" => Effect::IncreaseAttack,
            "increase-critical-rate" => Effect::IncreaseCriticalRate,
            "increase-defense" => Effect::IncreaseDefense,
            "increase-speed" => Effect::IncreaseSpeed,
            "invincibility" => Effect::Invincibility,
            "oblivion" => Effect::Oblivion,
            "protect-soul" => Effect::ProtectSoul,
            "provoke" => Effect::Provoke,
            "recovery" => Effect::Recovery,
            "reflect" => Effect::Reflect,
            "revenge" => Effect::Revenge,
            "shield" => Effect::Shield,
            "silence" => Effect::Silence,
            "sleep" => Effect::Sleep,
            "stun" => Effect::Stun,
            "threat" => Effect::Threat,
            "unrecoverable" => Effect::Unrecoverable,
            "vampire" => Effect::Vampire
        }
    }

}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub multiplier: Option<String>,
    pub image: String,
    pub skillups: Option<Vec<String>>,
    pub effects: Option<Vec<Effect>>
}

impl Skill {

    pub fn new(
        name: String, 
        description: String, 
        multiplier: Option<String>, 
        image: String,
        skillups: Option<Vec<String>>,
        effects: Option<Vec<String>>
    ) -> Self {
        Self {
            name,
            description,
            multiplier,
            image,
            skillups,
            effects: effects.map(|effects| {
                effects
                    .iter()
                    .map(|effect| {
                        Effect::from(effect)
                    })
            })
        }
    }

}