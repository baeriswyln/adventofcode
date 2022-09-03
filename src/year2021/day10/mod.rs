use std::collections::HashMap;

pub struct SyntaxScoring {
    data: Vec<String>,
}

impl crate::Advent for SyntaxScoring {
    fn new(data: &str) -> Self {
        SyntaxScoring {
            data: data
                .lines()
                .map(|line| String::from(line))
                .collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 10 }

    fn part1(&self) -> usize {
        let mut result = 0;
        let mut lookup = HashMap::new();
        lookup.insert(')', 3);
        lookup.insert(']', 57);
        lookup.insert('}', 1197);
        lookup.insert('>', 25137);

        for line in &self.data {
            let mut stack = Vec::new();

            for c in line.chars() {
                if ['(', '[', '{', '<'].contains(&c) { stack.push(c) } else {
                    match stack.pop().unwrap() {
                        '(' => if c != ')' { result += lookup.get(&c).unwrap() },
                        '[' => if c != ']' { result += lookup.get(&c).unwrap() },
                        '{' => if c != '}' { result += lookup.get(&c).unwrap() },
                        '<' => if c != '>' { result += lookup.get(&c).unwrap() },
                        _ => ()
                    }
                }
            }
        }

        result
    }

    fn part2(&self) -> usize {
        let mut scores = Vec::new();
        // create lookup table for getting corresponding parenthesis
        let mut lookup = HashMap::new();
        lookup.insert(')', '(');
        lookup.insert(']', '[');
        lookup.insert('}', '{');
        lookup.insert('>', '<');

        // create lookup table for getting the score of each parenthesis type
        let mut score_lookup = HashMap::new();
        score_lookup.insert('(', 1);
        score_lookup.insert('[', 2);
        score_lookup.insert('{', 3);
        score_lookup.insert('<', 4);

        'line: for line in &self.data {
            let mut stack = Vec::new();

            // add and remove parenthesis to/from the stack as long as there are more characters in
            // the line and there has the order is correct
            for c in line.chars() {
                if ['(', '[', '{', '<'].contains(&c) {
                    stack.push(c)
                } else {
                    let c1 = stack.pop().unwrap();
                    if *lookup.get(&c).unwrap() != c1 { continue 'line; }
                }
            }

            // if the stack is empty, cotninue to the next line
            if stack.len() == 0 { continue 'line; }

            // empty the stack and compute the score
            let mut score = 0;
            while stack.len() > 0 {
                score *= 5;
                score += match score_lookup.get(&stack.pop().unwrap()) {
                    Some(t) => *t,
                    None => continue 'line
                };
            }
            scores.push(score);
        }

        // get the score in the middle
        scores.sort();
        scores[(scores.len() - 1) / 2]
    }
}