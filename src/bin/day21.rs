fn main() {
    let input = std::fs::read_to_string("input/input21.txt").unwrap();
    println!("Solution1: {}", solution1(&input));
    println!("Solution2: {}", solution2(&input));
}

fn solution1(input: &str) -> i32 {
    let shop = Shop::new();
    let boss = Player::from(input);
    let mut min_price = i32::MAX;
    for bundle in shop.bundles() {
        //dbg!(&bundle);
        let mut boss_clone = boss.clone();
        let mut little_henry = Player::new();
        little_henry.hp = 100;
        little_henry.add_equipments(&bundle);
        if little_henry.fight_against(&mut boss_clone) {
            min_price = std::cmp::min(min_price, bundle.get_price());
        }
    }
    min_price
}

fn solution2(input: &str) -> i32 {
    let shop = Shop::new();
    let boss = Player::from(input);
    let mut max_price = i32::MIN;
    for bundle in shop.bundles() {
        //dbg!(&bundle);
        let mut boss_clone = boss.clone();
        let mut little_henry = Player::new();
        little_henry.hp = 100;
        little_henry.add_equipments(&bundle);
        if !little_henry.fight_against(&mut boss_clone) {
            max_price = std::cmp::max(max_price, bundle.get_price());
        }
    }
    max_price
}

#[derive(Debug, Clone)]
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

    fn add_equipments(&mut self, bundle: &Bundle) {
        self.damage += bundle.get_damage();
        self.armor += bundle.get_armor();
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

#[derive(Clone, Debug, PartialEq)]
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
        let weapons = vec![
            Equipment {
                name: String::from("Dagger"),
                price: 8,
                damage: 4,
                armor: 0,
            },
            Equipment {
                name: String::from("Shortsword"),
                price: 10,
                damage: 5,
                armor: 0,
            },
            Equipment {
                name: String::from("Warhammer"),
                price: 25,
                damage: 6,
                armor: 0,
            },
            Equipment {
                name: String::from("Longsword"),
                price: 40,
                damage: 7,
                armor: 0,
            },
            Equipment {
                name: String::from("Greataxe"),
                price: 74,
                damage: 8,
                armor: 0,
            },
        ];

        let armors = vec![
            Equipment {
                name: String::from("Leather"),
                price: 13,
                damage: 0,
                armor: 1,
            },
            Equipment {
                name: String::from("Chainmail"),
                price: 31,
                damage: 0,
                armor: 2,
            },
            Equipment {
                name: String::from("Splintmail"),
                price: 53,
                damage: 0,
                armor: 3,
            },
            Equipment {
                name: String::from("Bandedmail"),
                price: 75,
                damage: 0,
                armor: 4,
            },
            Equipment {
                name: String::from("Platemail"),
                price: 102,
                damage: 0,
                armor: 5,
            },
        ];

        let rings = vec![
            Equipment {
                name: String::from("Damage +1"),
                price: 25,
                damage: 1,
                armor: 0,
            },
            Equipment {
                name: String::from("Damage +2"),
                price: 50,
                damage: 2,
                armor: 0,
            },
            Equipment {
                name: String::from("Damage +3"),
                price: 100,
                damage: 3,
                armor: 0,
            },
            Equipment {
                name: String::from("Defense +1"),
                price: 20,
                damage: 0,
                armor: 1,
            },
            Equipment {
                name: String::from("Defense +2"),
                price: 40,
                damage: 0,
                armor: 2,
            },
            Equipment {
                name: String::from("Defense +3"),
                price: 80,
                damage: 0,
                armor: 3,
            },
        ];

        Shop {
            weapons,
            armors,
            rings,
        }
    }

    fn bundles(&self) -> BundleIter {
        BundleIter::new(self)
    }
}

#[derive(Debug, PartialEq)]
struct Bundle {
    equipments: Vec<Equipment>,
}

impl Bundle {
    fn get_damage(&self) -> i32 {
        self.equipments.iter().map(|e| e.damage).sum()
    }

    fn get_armor(&self) -> i32 {
        self.equipments.iter().map(|e| e.armor).sum()
    }

    fn get_price(&self) -> i32 {
        self.equipments.iter().map(|e| e.price).sum()
    }
}

struct BundleIter<'a> {
    weapon_idx: i32,
    armor_idx: i32,
    ring1_idx: i32,
    ring2_idx: i32,

    shop: &'a Shop,
}

impl<'a> BundleIter<'a> {
    fn new(shop: &Shop) -> BundleIter {
        BundleIter {
            weapon_idx: 0,
            armor_idx: -1,
            ring1_idx: -1,
            ring2_idx: -1,
            shop,
        }
    }

    fn increase_indices(&mut self) {
        self.ring2_idx += 1;
        if self.ring2_idx == self.shop.rings.len() as i32 {
            self.ring2_idx = -1;
            self.ring1_idx += 1;
            if self.ring1_idx == self.shop.rings.len() as i32 {
                self.ring1_idx = -1;
                self.armor_idx += 1;
                if self.armor_idx == self.shop.armors.len() as i32 {
                    self.armor_idx = -1;
                    self.weapon_idx += 1;
                }
            }
        } else if self.ring2_idx == self.ring1_idx {
            self.increase_indices();
        }
    }
}

impl<'a> Iterator for BundleIter<'a> {
    type Item = Bundle;

    fn next(&mut self) -> Option<Bundle> {
        if self.weapon_idx >= self.shop.weapons.len() as i32 {
            None
        } else {
            let mut equipments: Vec<Equipment> = Vec::new();
            equipments.push(self.shop.weapons[self.weapon_idx as usize].clone());
            if self.armor_idx >= 0 {
                equipments.push(self.shop.armors[self.armor_idx as usize].clone());
            }
            if self.ring1_idx >= 0 {
                equipments.push(self.shop.rings[self.ring1_idx as usize].clone());
            }
            if self.ring2_idx >= 0 {
                equipments.push(self.shop.rings[self.ring2_idx as usize].clone());
            }
            self.increase_indices();
            Some(Bundle { equipments })
        }
    }
}

#[test]
fn test_shop() {
    let shop = Shop::new();
    let mut iter = shop.bundles();
    assert_eq!(
        iter.next(),
        Some(Bundle {
            equipments: vec![shop.weapons[0].clone()]
        })
    );
    assert_eq!(
        iter.next(),
        Some(Bundle {
            equipments: vec![shop.weapons[0].clone(), shop.rings[0].clone()]
        })
    );
}
