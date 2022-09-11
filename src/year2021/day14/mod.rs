use std::collections::HashMap;

pub struct Polymerization {
    template: Vec<char>,
    rules: HashMap<String, char>,
}

impl crate::Advent for Polymerization {
    fn new(data: &str) -> Self {
        let mut split = data.split("\r\n\r\n");
        let mut rules = HashMap::new();

        let template = split.next().unwrap().chars().collect();

        split.next().unwrap().lines().for_each(|line| {
            let mut line_split = line.split(" -> ");
            rules.insert(
                line_split.next().unwrap().to_string(),
                line_split.next().unwrap().chars().next().unwrap(),
            );
        });

        Polymerization {
            template,
            rules,
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 14 }

    fn part1(&self) -> usize {
        let mut template = self.template.clone();

        for _ in 0..10 {
            template = insert_values(&template, &self.rules);
        }

        let mut counter = HashMap::new();

        template.iter().for_each(|c| {
            counter.entry(*c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });

        let max = *counter.values().max().unwrap();
        let min = *counter.values().min().unwrap();

        println!("{}-{}", max, min);
        max - min
    }

    fn part2(&self) -> usize {
        let mut counters: HashMap<String, (String, String, usize)> = HashMap::new();

        // compute the two pairs that will result when pushing a letter in between an existing pair
        for (key, &value) in &self.rules {
            let resulting_val_1 = format!("{}{}", key.chars().nth(0).unwrap(), value);
            let resulting_val_2 = format!("{}{}", value, key.chars().nth(1).unwrap());
            counters.insert((*key).clone(), (resulting_val_1, resulting_val_2, 0));
        }

        // initialize the counter with the template
        for i in 0..self.template.len() - 1 {
            counters.entry(format!("{}{}", self.template[i], self.template[i + 1]))
                .and_modify(|v| v.2 += 1);
        }

        for _ in 0..40 {
            counters = insert_values_fast(&counters);
        }

        let letter_counter = count_letters(&counters);

        let max = *letter_counter.values().max().unwrap()+1;
        let min = *letter_counter.values().min().unwrap()+1;

        println!("{}-{}", max, min);
        max - min
    }
}

fn insert_values(template: &Vec<char>, map: &HashMap<String, char>) -> Vec<char> {
    let mut new_template = Vec::new();

    for i in 0..template.len() - 1 {
        new_template.push(template[i]);

        match map.get(format!("{}{}", template[i], template[i + 1]).as_str()) {
            Some(t) => new_template.push(*t),
            None => ()
        }
    }

    new_template.push(template[template.len() - 1]);
    new_template
}

fn insert_values_fast(map: &HashMap<String, (String, String, usize)>)
                      -> HashMap<String, (String, String, usize)> {
    // clone the existing map
    let mut result = map.clone();

    // set all counters of the clone to 0
    for (_val1, _val2, count) in result.values_mut() {
        *count = 0;
    }

    // go through all values of the input counter map
    for (val1, val2, count) in map.values() {
        result.entry((*val1).clone())
            .and_modify(|val| val.2 += *count);
        result.entry((*val2).clone())
            .and_modify(|val| val.2 += *count);
    }

    result
}

fn count_letters(map: &HashMap<String, (String, String, usize)>) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    // count occurrences for each char
    for (key, (_, _, count)) in map {
        key.chars().for_each(|c| {
            result.entry((c).clone())
                .and_modify(|v| *v += *count)
                .or_insert(*count);
        });
    }

    // each letter is found double the amount it truly is in the final string. therefore, divide by
    // two. additionally, add 1 because the first and last letter are missing one letter to be
    // exactly double. integer division will make sure that "normal" letters are not affected
    result.iter_mut().for_each(|(_,v)| *v = (*v+1) / 2);

    result
}