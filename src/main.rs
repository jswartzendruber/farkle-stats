use rand::{thread_rng, Rng};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Die {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Die {
    fn roll() -> Die {
        let mut rng = thread_rng();
	rng.gen_range(1..7).into()
    }
}

impl From<usize> for Die {
    fn from(num: usize) -> Self {
	match num {
	    1 => Die::One,
            2 => Die::Two,
            3 => Die::Three,
            4 => Die::Four,
            5 => Die::Five,
            6 => Die::Six,
            _ => unreachable!(),
	}
    }
}

struct ScoreHelper {
    frequencies: Vec<usize>,
}

enum Special {
    ThreeKind,
    FourKind,
    FiveKind,
    SixKind,
    Straight,
    ThreePair,
    TwoTriplet,
    None,
}

impl ScoreHelper {
    fn new(hand: Vec<Die>) -> ScoreHelper {
        let mut dies = vec![0, 0, 0, 0, 0, 0];

        for die in &hand {
            let idx = *die as usize;
            dies[idx - 1] += 1;
        }

        ScoreHelper {
	    frequencies: dies,
        }
    }

    fn score(mut self) -> (usize, Special, usize) {
	// Two threes
	if self.frequencies.iter().filter(|x| x == &&3).count() == 2 {
	    return (2500, Special::TwoTriplet, 0);
	}

	// Three twos
	if self.frequencies.iter().filter(|x| x == &&2).count() == 3 {
	    return (1500, Special::ThreePair, 0);
	}

	// Straight
	if self.frequencies.iter().filter(|x| x == &&1).count() == 6 {
	    return (1500, Special::Straight, 0);
	}

	let mut score = 0;

	// X of a kind
	for index in 0..self.frequencies.len() {
	    let die_num = index + 1;
	    let freq = self.frequencies[index];

	    if freq >= 3 {
		let special = match freq {
		    3 => Special::ThreeKind,
		    4 => Special::FourKind,
		    5 => Special::FiveKind,
		    6 => Special::SixKind,
		    _ => unreachable!(),
		};
		score += Self::x_of_a_kind(freq, die_num.into());
		self.frequencies[index] = 0; // These dice are used up for these points
	    }
	}

	// Ones and fives
	score += 100 * self.frequencies[0];
	self.frequencies[0] = 0;
	score += 50 * self.frequencies[4];
	self.frequencies[4] = 0;

	let non_counters = self.frequencies.iter().filter(|x| x == &&0).count();

	(score, Special::None, non_counters)
    }

    fn x_of_a_kind(frequency: usize, die: Die) -> usize {
	match frequency {
            3 => match die {
		Die::One => 300,
		Die::Two => 200,
		Die::Three => 300,
		Die::Four => 400,
		Die::Five => 500,
		Die::Six => 600,
            },
            4 => 1000,
            5 => 2000,
            6 => 3000,
            _ => unreachable!(),
	}
    }
}

struct Farkle {
    dice_to_roll: usize,
}

impl Farkle {
    fn roll(&self) -> (usize, Special, usize) {
        let mut hand = vec![];

        for _ in 0..self.dice_to_roll {
            hand.push(Die::roll())
        }

        ScoreHelper::new(hand).score()
    }
}

fn main() {
    // let times = 1_000_000;

    // let mut big_score = 0;
    // let mut turns = 0;

    // for _ in 0..times {
    // 	let game_score = 10_000;
    // 	let mut curr_score = 0;
    // 	while curr_score < game_score {
    // 	    turns += 1;
    // 	    let farkle = Farkle { dice_to_roll: 6 };
    // 	    let (score, special, non_counters) = farkle.roll();
    // 	    curr_score += score;
    // 	}
    // 	big_score += curr_score;
    // }

    // println!("Average score per turn: {}", big_score as f64 / turns as f64);

    // Roll only first roll
    let times = 1_000_000;
    let mut turns = 0;
    let mut score_all_games = 0;

    for _ in 0..times {
	let game_score = 10_000;
	let mut curr_score = 0;
	while curr_score < game_score {
	    let mut temp_score = 0;
	    let (mut score, mut special, mut non_counters) = (1, Special::None, 6);

	    while non_counters >= 6 {
		turns += 1;
		let f = Farkle { dice_to_roll: non_counters };
		(score, special, non_counters) = f.roll();
		if score == 0 {
		    continue; // No points
		} else {
		    temp_score += score;
		}
	    }

	    curr_score += temp_score;
	}
	score_all_games += curr_score;
    }

    println!("Average score: {:.3}", score_all_games as f64 / turns as f64);

    // let times = 10_000_000;

    // let mut two_trip = 0;
    // let mut three_pair = 0;
    // let mut straight = 0;
    // let mut three = 0;
    // let mut four = 0;
    // let mut five = 0;
    // let mut six = 0;
    // let mut zero = 0;

    // for _ in 0..times {
    // 	let farkle = Farkle { dice_to_roll: 1 };
    // 	let (score, special) = farkle.roll();

    // 	match special {
    // 	    Special::TwoTriplet => two_trip += 1,
    // 	    Special::ThreePair => three_pair += 1,
    // 	    Special::Straight => straight += 1,
    // 	    Special::ThreeKind => three += 1,
    // 	    Special::FourKind => four += 1,
    // 	    Special::FiveKind => five += 1,
    // 	    Special::SixKind => six += 1,
    // 	    Special::None => {}
    // 	}

    // 	if score == 0 {
    // 	    zero += 1;
    // 	}
    // }

    // println!("zero : {:.3}%", (zero as f64 / times as f64) * 100.0);
    // println!("two_trip : {:.3}%", (two_trip as f64 / times as f64) * 100.0);
    // println!("three_pair : {:.3}%", (three_pair as f64 / times as f64) * 100.0);
    // println!("straight : {:.3}%", (straight as f64 / times as f64) * 100.0);
    // println!("three : {:.3}%", (three as f64 / times as f64) * 100.0);
    // println!("four : {:.3}%", (four as f64 / times as f64) * 100.0);
    // println!("five : {:.3}%", (five as f64 / times as f64) * 100.0);
    // println!("six : {:.3}%", (six as f64 / times as f64) * 100.0);
}
