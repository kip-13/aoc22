use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum HandShapeValues {
    None = 0,
    Rock,
    Paper,
    Scissors
}

enum Result {
    Invalid,
    Win,
    Lost,
    Draw
}

impl Result {
    pub fn points(&self) -> i32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lost => 0,
            Result::Invalid => -1
        }
    }
}

struct Rock { p: HandShapeValues }
struct Paper { p: HandShapeValues }
struct  Scissors { p: HandShapeValues }

struct  Draw { p: HandShapeValues }
struct  Lose { p: HandShapeValues }
struct  Win { p: HandShapeValues }

trait HandShapeOption {
    fn new() -> Self;
    fn vs(&mut self, vs: &HandShape) -> Result;
}

impl HandShapeOption for Rock {
     fn new() -> Rock {
        Rock{p: HandShapeValues::Rock}
    }

     fn vs(&mut self, vs: &HandShape) -> Result {
        match vs {
            HandShape::Paper(_) => Result::Lost,
            HandShape::Scissors(_) => Result::Win,
            HandShape::Rock(_) => Result::Draw,
            _ => Result::Invalid
        }
    }
}

impl HandShapeOption for Paper {
    fn new() -> Paper {
        Paper{p: HandShapeValues::Paper}
    }

    fn vs(&mut self, vs: &HandShape) -> Result {
        match vs {
            HandShape::Paper(_) => Result::Draw,
            HandShape::Scissors(_) => Result::Lost,
            HandShape::Rock(_) => Result::Win,
            _ => Result::Invalid
        }
    }
}

impl HandShapeOption for Scissors {
     fn new() -> Scissors {
        Scissors{p: HandShapeValues::Scissors}
    }

     fn vs(&mut self, vs: &HandShape) -> Result {
        match vs {
            HandShape::Paper(_) => Result::Win,
            HandShape::Scissors(_) => Result::Draw,
            HandShape::Rock(_) => Result::Lost,
            _ => Result::Invalid
        }
    }
}

impl HandShapeOption for Draw {
     fn new() ->  Draw {
        Draw{p: HandShapeValues::None}
    }

     fn vs(&mut self, vs: &HandShape) -> Result {
         match vs {
            HandShape::Paper(_) => self.p = HandShapeValues::Paper,
            HandShape::Scissors(_) => self.p = HandShapeValues::Scissors,
            HandShape::Rock(_) => self.p = HandShapeValues::Rock,
            _ => self.p = self.p
        }

        Result::Draw
    }
}

impl HandShapeOption for Lose {
     fn new() ->  Lose {
        Lose{p: HandShapeValues::None}
    }

     fn vs(&mut self, vs: &HandShape) -> Result {
         match vs {
            HandShape::Paper(_) => self.p = HandShapeValues::Rock,
            HandShape::Scissors(_) => self.p = HandShapeValues::Paper,
            HandShape::Rock(_) => self.p = HandShapeValues::Scissors,
            _ => self.p = self.p
        }

        Result::Lost
    }
}

impl HandShapeOption for Win {
     fn new() ->  Win {
        Win{p: HandShapeValues::None}
    }

     fn vs(&mut self, vs: &HandShape) -> Result {
         match vs {
            HandShape::Paper(_) => self.p = HandShapeValues::Scissors,
            HandShape::Scissors(_) => self.p = HandShapeValues::Rock,
            HandShape::Rock(_) => self.p = HandShapeValues::Paper,
            _ => self.p = self.p
        }

        Result::Win
    }
}

enum HandShape
{
    Rock(Rock),
    Paper(Paper),
    Scissors(Scissors),
    Draw(Draw),
    Lose(Lose),
    Win(Win)
}

impl HandShape {
    fn wrong_strategy(from: &str) -> HandShape {
        match from {
            "A" | "X" => HandShape::Rock(Rock::new()),
            "B" | "Y" => HandShape::Paper(Paper::new()),
            "C" | "Z" => HandShape::Scissors(Scissors::new()),
            &_ => panic!("bad")
        }
    }

    fn strategy(from: &str) -> HandShape {
        match from {
            "A" => HandShape::Rock(Rock::new()),
            "B" => HandShape::Paper(Paper::new()),
            "C" => HandShape::Scissors(Scissors::new()),
            "X" => HandShape::Lose(Lose::new()),
            "Y" => HandShape::Draw(Draw::new()),
            "Z" => HandShape::Win(Win::new()),
            &_ => panic!("bad")
        }
    }

    pub fn vs(&mut self, vs: &HandShape) -> Points {
        match self {
            HandShape::Paper(r) => Points { hand_shape: r.vs(vs), vs_match: r.p as i32 },
            HandShape::Scissors(r) => Points { hand_shape: r.vs(vs), vs_match: r.p  as i32 },
            HandShape::Rock(r) => Points { hand_shape: r.vs(vs), vs_match: r.p  as i32 },
            HandShape::Win(r) => Points { hand_shape: r.vs(vs), vs_match: r.p as i32 },
            HandShape::Lose(r) => Points { hand_shape: r.vs(vs), vs_match: r.p  as i32 },
            HandShape::Draw(r) => Points { hand_shape: r.vs(vs), vs_match: r.p  as i32 },
        }
    }
}

struct Points {
    hand_shape: Result,
    vs_match: i32
}

impl Points {
    pub fn total(&self) -> i32 {
        self.hand_shape.points() + self.vs_match
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sol1 = 0;
    let mut sol2 = 0;

    for line in stdin.lock().lines() {
        let l = line.unwrap();

        let items : Vec<&str> = l.split(" ").collect();

        let p1 = HandShape::strategy(items[0]);
        let mut p2_wrong_strategy = HandShape::wrong_strategy(items[1]);
        let mut p2_strategy = HandShape::strategy(items[1]);

        let result_1 = p2_wrong_strategy.vs(&p1);
        let result_2 = p2_strategy.vs(&p1);

        sol1 += result_1.total();
        sol2 += result_2.total();
    }
    
    println!("sol 1: {}", sol1);
    println!("sol 2: {}", sol2);

}
