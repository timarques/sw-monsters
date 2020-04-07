use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Value {
    name: String,
    value: i32,
    percentage: bool
}

impl Value {

    pub fn new(name: &str, value: i32, percentage: bool) -> Self {
        Self {
            name: name.to_string(),
            value,
            percentage
        }
    }

}

struct Builder {
    speed: i32,
    critical_rate: i32,
    critical_damage: i32,
    resistance: i32,
    accuracy: i32,
    health: i32,
    attack: i32,
    defense: i32
}

impl Builder {

    pub fn new() -> Self {
        Self {
            speed: 0,
            critical_rate: 15,
            critical_damage: 50,
            resistance: 15,
            accuracy: 0,
            health: 0,
            attack: 0,
            defense: 0
        }
    }

    pub fn speed(mut self, value: i32) -> Self {
        self.speed = value;
        self
    }

    pub fn critical_rate(mut self, value: i32) -> Self {
        self.critical_rate = value;
        self
    }

    pub fn critical_damage(mut self, value: i32) -> Self {
        self.critical_damage = value;
        self
    }

    pub fn resistance(mut self, value: i32) -> Self {
        self.resistance = value;
        self
    }

    pub fn accuracy(mut self, value: i32) -> Self {
        self.accuracy = value;
        self
    }

    pub fn health(mut self, value: i32) -> Self {
        self.health = value;
        self
    }

    pub fn attack(mut self, value: i32) -> Self {
        self.attack = value;
        self
    }

    pub fn defense(mut self, value: i32) -> Self {
        self.defense = value;
        self
    }

    pub fn build(self) -> Stats {
        Stats::new(
            self.speed,
            self.critical_rate,
            self.critical_damage,
            self.resistance,
            self.accuracy,
            self.health,
            self.attack,
            self.defense
        )
    }

}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Stats {
    pub speed: Value,
    pub critical_rate: Value,
    pub critical_damage: Value,
    pub resistance: Value,
    pub accuracy: Value,
    pub health: Value,
    pub attack: Value,
    pub defense: Value
}

impl Stats {

    pub fn new(
        speed: i32,
        critical_rate: i32,
        critical_damage: i32,
        resistance: i32,
        accuracy: i32,
        health: i32,
        attack: i32,
        defense: i32
    ) -> Self {
        Self{
            speed: Value::new("Speed", speed, false),
            critical_rate: Value::new("Critical Rate", critical_rate, true),
            critical_damage: Value::new("Critical Damage", critical_damage, true),
            resistance: Value::new("Resistance", resistance, true),
            accuracy: Value::new("Accuracy", accuracy, true),
            health: Value::new("Health", health, false),
            attack: Value::new("Attack", attack, false),
            defense: Value::new("Defense", defense, false)
        }
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn empty() -> Self {
        Self {
            speed: Value::new("", 0, false),
            critical_rate: Value::new("", 0, false),
            critical_damage: Value::new("", 0, false),
            resistance: Value::new("", 0, false),
            accuracy: Value::new("", 0, false),
            health: Value::new("", 0, false),
            attack: Value::new("", 0, false),
            defense: Value::new("", 0, false)
        }
    }

}
