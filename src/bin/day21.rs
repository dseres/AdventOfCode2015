fn main() {
    let input = std::fs::read_to_string("input/input21.txt").unwrap();
    let mut boss = Player::from(&input);
    let mut little_henry = Player::new();
    little_henry.hp = 100;
}

struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Player {
    fn new() -> Player {
        Player {
            hp: 0,
            damage: 0,
            armor: 0,
        }
    }

    fn from(input: &str) -> Player {
        let mut lines = input.lines();
        let hp = Self::read_number(&mut lines);
        let damage = Self::read_number(&mut lines);
        let armor = Self::read_number(&mut lines);
        Player { hp, damage, armor }
    }

    fn read_number(lines: &mut std::str::Lines) -> i32 {
        let mut splitted = lines.next().unwrap().split(": ");
        splitted.next();
        let second_part = splitted.next().unwrap();
        second_part.parse::<i32>().unwrap()
    }

    fn apply_damage(&mut self, damage: i32) {
        let mut damage = damage - self.armor;
        if damage <= 0 {
            damage = 1;
        }
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    fn is_died(&self) -> bool {
        self.hp <= 0
    }

    // Returns true if actual player wins
    fn fight_against(&mut self, other: &mut Player) -> bool {
        loop {
            other.apply_damage(self.damage);
            if other.is_died() {
                return true;
            }
            self.apply_damage(other.damage);
            if self.is_died() {
                return false;
            }
        }
    }
}

#[test]
fn test_fight_against() {
    let mut boss = Player {
        hp: 12,
        damage: 7,
        armor: 2,
    };
    let mut little_henry = Player {
        hp: 8,
        damage: 5,
        armor: 5,
    };
    assert!(little_henry.fight_against(&mut boss));
    assert_eq!(little_henry.hp, 2);
}

struct Equipment {
    name: String,
    price: i32,
    damage: i32,
    armor: i32,
}

struct Shop {
    weapons: Vec<Equipment>,
    armors: Vec<Equipment>,
    rings: Vec<Equipment>,
}

impl Shop {
    fn new() -> Shop {
        let mut weapons = Vec::new();
        weapons.push(Equipment {
            name: String::from("Dagger"),
            price: 8,
            damage: 4,
            armor: 0,
        });
        weapons.push(Equipment {
            name: String::from("Shortsword"),
            price: 10,
            damage: 5,
            armor: 0,
        });
        weapons.push(Equipment {
            name: String::from("Warhammer"),
            price: 25,
            damage: 6,
            armor: 0,
        });
        weapons.push(Equipment {
            name: String::from("Longsword"),
            price: 40,
            damage: 7,
            armor: 0,
        });
        weapons.push(Equipment {
            name: String::from("Greataxe"),
            price: 74,
            damage: 8,
            armor: 0,
        });

        let mut armors = Vec::new();
        armors.push( Equipment { name: String::from("Leather"), price: 13, damage: 0, armor: 1 });
        armors.push( Equipment { name: String::from("Chainmail"), price: 31, damage: 0, armor: 2 });
        armors.push( Equipment { name: String::from("Splintmail"), price: 53, damage: 0, armor: 3 });
        armors.push( Equipment { name: String::from("Bandedmail"), price: 75, damage: 0, armor: 4 });
        armors.push( Equipment { name: String::from("Platemail"), price: 102, damage: 0, armor: 5 });

        let mut rings = Vec::new();
        rings.push( Equipment { name: String::from("Damage +1"), price: 25, damage: 1, armor: 0 });
        rings.push( Equipment { name: String::from("Damage +2"), price: 50, damage: 2, armor: 0 });
        rings.push( Equipment { name: String::from("Damage +3"), price: 100, damage: 3, armor: 0 });
        rings.push( Equipment { name: String::from("Defense +1"), price: 20, damage: 0, armor: 1 });
        rings.push( Equipment { name: String::from("Defense +2"), price: 40, damage: 0, armor: 2 });
        rings.push( Equipment { name: String::from("Defense +3"), price: 80, damage: 0, armor: 3 });

        Shop {
            weapons,
            armors,
            rings,
        }
    }
}
