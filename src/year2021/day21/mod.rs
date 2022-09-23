struct Board {
    players: Vec<Player>,
    next_player: usize,
    dice: Dice,
    size: usize,
    max_score: usize,
}

impl Board {
    fn play(&mut self) -> bool {
        let roll = self.dice.roll_deterministic(3);
        self.players[self.next_player].position = (self.players[self.next_player].position + roll) % self.size;
        self.players[self.next_player].score += self.players[self.next_player].position + 1;

        let result = self.players[self.next_player].score >= self.max_score;
        println!("Player {} rolls {} and moves to space {} for a total score of {}", self.next_player+1, roll, self.players[self.next_player].position+1, self.players[self.next_player].score);
        self.next_player = (self.next_player + 1) % self.players.len();
        result
    }
}

struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Player {
            score: 0,
            position,
        }
    }
}

struct Dice {
    last_value: usize,
    rolls: usize,
    sides: usize,
}

impl Dice {
    fn new(sides: usize) -> Self {
        Dice {
            last_value: 0,
            rolls: 0,
            sides,
        }
    }

    fn roll_deterministic(&mut self, n: usize) -> usize {
        let mut result = 0;
        for _ in 0..n {
            result += self.last_value + 1;
            self.rolls += 1;
            self.last_value = (self.last_value + 1) % self.sides;
        }
        result
    }
}

pub struct DiracDice {
    start_1: usize,
    start_2: usize,
}

impl crate::Advent for DiracDice {
    fn new(data: &str) -> Self {
        let mut iter = data.lines();
        DiracDice {
            start_1: iter.next().unwrap().chars().nth(28).unwrap() as usize - 48,
            start_2: iter.next().unwrap().chars().nth(28).unwrap() as usize - 48,
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 21 }

    fn part1(&self) -> usize {
        let mut board = Board {
            players: vec![Player::new(self.start_1-1), Player::new(self.start_2-1)],
            next_player: 0,
            dice: Dice::new(100),
            size: 10,
            max_score: 1000,
        };

        while !board.play() {}

        println!("rolls: {}", board.dice.rolls);
        println!("score: {}", board.players[board.next_player].score);

        board.dice.rolls * board.players[board.next_player].score
    }

    fn part2(&self) -> usize {
        0
    }
}