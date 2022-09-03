use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("./input/input14.txt").unwrap();
    let mut reindeers = get_reindeers(&contents);
    for rd in &reindeers {
        println!("{:?}", rd);
    }
    simulate_race1(2503, &mut reindeers);
    let max_dist = reindeers.iter().map(|rd| rd.distance).max().unwrap();
    println!("Solution1: {}", max_dist);

    simulate_race2(2503, &mut reindeers);
    let max_point = reindeers.iter().map(|rd| rd.points).max().unwrap();
    println!("Solution2: {}", max_point);
}

fn get_reindeers(input: &str) -> Vec<ReinDeer> {
    let mut reindeers: Vec<ReinDeer> = Vec::new();
    for line in input.lines() {
        reindeers.push(ReinDeer::from_line(line));
    }
    reindeers
}

fn simulate_race1(time: i32, reindeers: &mut [ReinDeer]) {
    for rd in reindeers.iter_mut() {
        rd.start_race();
    }
    for _t in 0..time {
        for rd in reindeers.iter_mut() {
            rd.tick();
        }
    }
}

fn simulate_race2(time: i32, reindeers: &mut [ReinDeer]) {
    for rd in reindeers.iter_mut() {
        rd.start_race();
    }
    for _t in 0..time {
        let mut max_dist = 0;
        for rd in reindeers.iter_mut() {
            rd.tick();
            if rd.distance > max_dist {
                max_dist = rd.distance;
            }
        }
        for rd in reindeers.iter_mut().filter(|rd| rd.distance == max_dist) {
            rd.points += 1;
        }
    }
}

#[derive(Debug)]
enum ReinDeerState {
    Flying(i32),
    Resting(i32),
    DoingNothing,
}

#[derive(Debug)]
struct ReinDeer {
    name: String,
    speed: i32,
    flying_time: i32,
    resting_time: i32,
    points: i32,
    distance: i32,
    state: ReinDeerState,
}

impl ReinDeer {
    fn new() -> ReinDeer {
        ReinDeer {
            name: String::new(),
            speed: 0,
            flying_time: 0,
            resting_time: 0,
            points: 0,
            distance: 0,
            state: ReinDeerState::DoingNothing,
        }
    }

    fn from_line(line: &str) -> ReinDeer {
        lazy_static::lazy_static! {
            static ref REG: Regex = Regex::new(r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$").unwrap();
        }
        let caps = REG.captures(line).unwrap();
        let mut rd = ReinDeer::new();
        rd.name = String::from(caps.get(1).unwrap().as_str());
        rd.speed = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        rd.flying_time = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        rd.resting_time = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        rd
    }

    fn start_race(&mut self) {
        self.points = 0;
        self.distance = 0;
        self.state = ReinDeerState::Flying(self.flying_time);
    }

    fn tick(&mut self) {
        match self.state {
            ReinDeerState::Flying(t) => self.fly(t),
            ReinDeerState::Resting(t) => self.rest(t),
            _ => panic!(),
        }
    }

    fn fly(&mut self, t: i32) {
        self.distance += self.speed;
        if t > 1 {
            self.state = ReinDeerState::Flying(t - 1);
        } else {
            self.state = ReinDeerState::Resting(self.resting_time);
        }
    }

    fn rest(&mut self, t: i32) {
        if t > 1 {
            self.state = ReinDeerState::Resting(t - 1);
        } else {
            self.state = ReinDeerState::Flying(self.flying_time);
        }
    }
}
