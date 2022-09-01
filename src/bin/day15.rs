use aoc2015::input;
use aoc2015::input::ReadFromLine;
use regex::Regex;

fn main() {
    let ingredients = input::read_input_file::<Ingredient>("./input/input15.txt");
    println!("{:?}", &ingredients);
    let mut p = Compositor::new(100, ingredients.len());
    p.iterate_recipes(ingredients.clone());
    println!("Solution1: {}", p.best1);
    println!("Solution2: {}", p.best2);
}

#[derive(Debug, Clone, Default)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl ReadFromLine<Ingredient> for Ingredient {
    fn from_line(line: &str) -> Ingredient {
        lazy_static::lazy_static! {
            static ref REG: Regex = Regex::new(r"^(\w+): capacity ([\-0-9]+), durability ([\-0-9]+), flavor ([\-0-9]+), texture ([\-0-9]+), calories ([\-0-9]+)$").unwrap();
        }
        let caps = REG.captures(&line).unwrap();
        let name = String::from(caps.get(1).unwrap().as_str());
        let capacity = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let durability = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let flavor = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let texture = caps.get(5).unwrap().as_str().parse::<i32>().unwrap();
        let calories = caps.get(6).unwrap().as_str().parse::<i32>().unwrap();
        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

impl Ingredient {
    fn compose_to_recipe(ingredients: &[Ingredient], composition: &[i32]) -> Ingredient {
        let mut recipe = Ingredient {
            name: String::from("Recipe"),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };
        for (i, _item) in composition.iter().enumerate() {
            recipe.capacity += composition[i] * ingredients[i].capacity;
            recipe.durability += composition[i] * ingredients[i].durability;
            recipe.flavor += composition[i] * ingredients[i].flavor;
            recipe.texture += composition[i] * ingredients[i].texture;
            recipe.calories += composition[i] * ingredients[i].calories;
        }
        recipe.capacity = std::cmp::max(recipe.capacity, 0);
        recipe.durability = std::cmp::max(recipe.durability, 0);
        recipe.flavor = std::cmp::max(recipe.flavor, 0);
        recipe.texture = std::cmp::max(recipe.texture, 0);
        recipe
    }

    fn get_point_and_calories(&self) -> (i32, i32) {
        let mut point = self.capacity * self.durability * self.flavor * self.texture;
        point = std::cmp::max(0, point);
        (point, self.calories)
    }
}

#[derive(Debug)]
struct Compositor {
    sum: i32,
    len: usize,
    ingredients: Vec<Ingredient>,
    best1: i32,
    best2: i32,
}

impl Compositor {
    fn new(sum: i32, len: usize) -> Compositor {
        Compositor {
            sum,
            len,
            ingredients: Vec::new(),
            best1: 0,
            best2: 0,
        }
    }

    //Recursive and simple function
    #[allow(dead_code)]
    fn iterate_recipes(&mut self, ingredients: Vec<Ingredient>) {
        self.best1 = 0;
        self.best2 = 0;
        self.ingredients = ingredients;
        let mut composition: Vec<i32> = vec![0; self.len];
        composition[0] = self.sum;
        self.iterate_recipes_impl(0, composition);
    }

    fn iterate_recipes_impl(&mut self, index: usize, composition: Vec<i32>) {
        self.check_recipe(&composition);
        if index >= composition.len() - 1 {
            return;
        }
        for i in 1..=composition[index] {
            let mut c = composition.clone();
            c[index] -= i;
            c[index + 1] += i;
            self.iterate_recipes_impl(index + 1, c);
        }
    }

    fn check_recipe(&mut self, composition: &[i32]) {
        let recipe = Ingredient::compose_to_recipe(&self.ingredients, &composition);
        let (point, calories) = recipe.get_point_and_calories();
        if point > self.best1 {
            self.best1 = point
        }
        if point > self.best2 && calories == 500 {
            self.best2 = point
        }
    }
}
