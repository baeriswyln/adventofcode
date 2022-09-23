use std::cmp::max;
use std::collections::HashMap;

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
            players: vec![Player::new(self.start_1 - 1), Player::new(self.start_2 - 1)],
            next_player: 0,
            dice: Dice::new(100),
            size: 10,
            max_score: 1000,
        };

        while !board.play() {}

        board.dice.rolls * board.players[board.next_player].score
    }

    /// linear approach for an exponential growing number of possibilities.
    fn part2(&self) -> usize {
        // these hashmaps are used to store the number of universes currently playing. each universe
        // is designated by the position and score of both players, giving a key set with 4 values.
        let mut h: HashMap<(usize, usize, usize, usize), usize> = HashMap::new();
        let mut h_next: HashMap<(usize, usize, usize, usize), usize>;

        // counting the number of universes in which the respective player wins
        let mut player_1_wins = 0;
        let mut player_2_wins = 0;

        // temporary variable
        let mut temp;

        // store all possible throw values for 3 dice throws with values between 1 and 3
        let mut possible_steps = Vec::new();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    possible_steps.push(i + j + k);
                }
            }
        }

        // initialize states. subtract 1 from initial position as input starts counting at 1, but we
        // are working with values between 0 and 9 for board position
        h.insert((self.start_1 - 1, 0, self.start_2 - 1, 0), 1);

        // repeat until all player finished
        while h.len() > 0 {
            // play player 1. each state will undergo all of the 27 possible dice throws.
            // the number of states currently in state a will be added to the number of states for
            // the next turn by moving all of the 27 possible step sizes.
            h_next = HashMap::new();
            for (&key, &vals) in &h {
                for &step in &possible_steps {
                    h_next.entry(((key.0 + step) % 10, (key.0 + step) % 10 + 1 + key.1,
                                  key.2, key.3))
                        .and_modify(|v| *v += vals)
                        .or_insert(vals);
                }
            }

            // check all scores
            (temp, h) = remove_finished(&h_next);
            player_1_wins += temp;

            // play player 2. repeat the same as before but for player 2
            h_next = HashMap::new();
            for (&key, &vals) in &h {
                for &step in &possible_steps {
                    h_next.entry((key.0, key.1, (key.2 + step) % 10,
                                  (key.2 + step) % 10 + 1 + key.3))
                        .and_modify(|v| *v += vals)
                        .or_insert(vals);
                }
            }

            // check all scores
            (temp, h) = remove_finished(&h_next);
            player_2_wins += temp;
        }

        println!("{}", player_1_wins);
        println!("{}", player_2_wins);

        max(player_1_wins, player_2_wins)
    }
}

/// check all scores and remove the states that have scores >= 21.
fn remove_finished(h: &HashMap<(usize, usize, usize, usize), usize>)
                   -> (usize, HashMap<(usize, usize, usize, usize), usize>) {
    let mut counter = 0;
    let mut states = HashMap::new();

    for (&key, &state) in h {
        if key.1 >= 21 || key.3 >= 21 {
            counter += state;
        } else {
            states.insert(key, state);
        }
    }

    (counter, states)
}