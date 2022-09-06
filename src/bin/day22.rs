use std::ops::Index;

fn main(){
    let input = std::fs::read_to_string("input/input22.txt").unwrap();
    println!("Solution1: {}", solution1(&input));
    println!("Solution2: {}", solution2(&input));
}

fn solution1(input: &str) -> i32 {
    let boss = Player::read_boss(input);
    let henry = Player::create_little_henry();
    0
}

fn solution2(input: &str) -> i32 {
    let boss = Player::read_boss(input);
    let henry = Player::create_little_henry();
    0
}


#[derive(Debug, Clone)]
struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
    mana: i32,
    spells : Vec<Spell>,
    effects : Vec<Spell>,
}

impl Player {
    fn new() -> Player {
        Player {
            hp: 0,
            damage: 0,
            armor: 0,
            mana: 0,
            spells: Vec::new(),
            effects: Vec::new()
        }
    }

    fn read_boss(input: &str) -> Player {
        let mut lines = input.lines();
        let hp = Self::read_number(&mut lines);
        let damage = Self::read_number(&mut lines);
        Player { hp, damage, armor:0 , mana: 0, spells: Vec::new(), effects: Vec::new()}
    }

    fn create_little_henry() -> Player {
        Player{ hp: 50, damage: 0, armor: 0, mana: 500, spells : create_spells(), effects: Vec::new()}
    }

    fn read_number(lines: &mut std::str::Lines) -> i32 {
        let mut splitted = lines.next().unwrap().split(": ");
        splitted.next();
        let second_part = splitted.next().unwrap();
        second_part.parse::<i32>().unwrap()
    }

    fn apply_damage(&mut self, damage: i32) {
        let mut damage = damage-self.armor;
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
    fn check_one_turn(&mut self, other: &mut Player) -> Option<i32> {
        self.apply_effects(other);
        if other.is_died(){
            return Some(0);
        }
        let mut min_mana = i32::MAX;
        for spell_idx in 0..self.spells.len(){
            let spell = &self.spells[spell_idx];
            if !self.can_cast(spell){
                continue;
            }
            let mut used_mana = spell.cost;
            let mut new_player = self.clone();
            let mut new_boss = other.clone();
            new_player.cast_spell(&mut new_boss, spell);
            if new_boss.is_died(){
                min_mana = std::cmp::min(min_mana, used_mana);
            } else {
                new_player.check_one_turn(&mut new_boss);
            }
        }
        self.apply_effects(other);
        if other.is_died(){
            return Some(min_mana);
        }
        self.apply_damage(other.damage);
        if self.is_died() {
            return None;
        }
        Some(min_mana)
    }

    fn can_cast(&self, spell: &Spell)->bool{
        spell.cost <= self.mana
    }

    fn cast_spell(&mut self, other: &mut Player, spell: &Spell){
        if !spell.is_effect(){
            self.mana -= spell.cost;
            self.hp += spell.healing;
            other.apply_damage(spell.damage);
        } else {
            //apply effect
            if self.is_effect_applicable(spell){
                self.effects.push(spell.clone());
            }
        }
    }

    fn is_effect_applicable(&self, spell: &Spell)->bool{
        !self.effects.iter().any(|s| s.name == spell.name )
    }

    fn apply_effects(&mut self, other: &mut Player){
        for i in 0..self.effects.len(){
            self.apply_effect(other, i);
        }
    }

    fn apply_effect(&mut self, other: &mut Player, effect_idx: usize){
        let mut effect = &self.effects[effect_idx];
        if effect.damage > 0{
            other.apply_damage(effect.damage)
        }
        if effect.armor >0{
            self.armor = effect.armor
        }
        if effect.mana > 0 {
            self.mana += effect.mana;
        }
        self.handle_turn(effect_idx);
    }

    fn handle_turn(&mut self, effect_idx: usize){
        self.effects[effect_idx].turns -= 1;
        if self.effects[effect_idx].turns == 0 {
            self.effects.remove(effect_idx);
        }
    }

}

#[derive(Debug, Clone)]
struct Spell {
    name : String,
    cost : i32,
    damage : i32,
    healing : i32,
    mana : i32,
    armor : i32,
    turns : i32,
}

impl Spell {
    fn is_effect(&self)->bool{
        self.turns>0
    }
}

fn create_spells()->Vec<Spell>{
    vec![
        Spell{ name: String::from("Macig missile"), cost: 53, damage: 4, healing : 0, mana: 0, armor: 0, turns : 0},
        Spell{ name: String::from("Drain"), cost: 73, damage: 2, healing : 2, mana: 0, armor: 0, turns : 0},
        Spell{ name: String::from("Shield"), cost: 113, damage: 0, healing : 0, mana: 0, armor: 7, turns : 6},
        Spell{ name: String::from("Poison"), cost: 173, damage: 3, healing : 0, mana: 0, armor: 0, turns : 6},
        Spell{ name: String::from("Recharge"), cost: 229, damage: 4, healing : 0, mana: 101, armor: 0, turns : 5},
    ]
}
