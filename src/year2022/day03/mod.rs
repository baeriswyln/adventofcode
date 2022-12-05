use std::collections::HashMap;

pub struct RucksackReorganization {
    data: Vec<String>,
}

impl crate::Advent for RucksackReorganization {
    fn new(data: &str) -> Self {
        RucksackReorganization {
            data: data.lines().map(|l| l.to_string()).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        3
    }

    fn part1(&self) -> usize {
        self.data.iter().map(|l| {
            let c = l.len();
            let mut chars = HashMap::new();
            let mut iter = l.chars();

            for _ in 0..c / 2 {
                let v = iter.next().unwrap() as usize;
                chars.insert(v, v);
            }

            for _ in c / 2..c {
                match chars.get(&(iter.next().unwrap() as usize)) {
                    Some(x) => {
                        return if *x < ('a' as usize) {
                            *x - ('A' as usize) + 27
                        } else {
                            *x - ('a' as usize) + 1
                        };
                    }
                    None => ()
                }
            }
            0
        }).sum()
    }

    fn part2(&self) -> usize {
        let mut iter = self.data.iter().peekable();
        let mut result = 0;

        loop {
            let mut map = HashMap::new();
            for _ in 0..3 {
                let mut sub_map = HashMap::new();
                iter.next().unwrap().chars().for_each(|c| {
                    sub_map.entry(c)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                });

                sub_map.iter().for_each(|(&k, _)| {
                    map.entry(k)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                });
            }

            for (&k, &v) in map.iter() {
                if v == 3 {
                    result += if (k as usize)<('a' as usize) {
                        k as usize - ('A' as usize) + 27
                    } else {
                        k as usize - ('a' as usize) + 1
                    };
                }
            }

            if iter.peek().is_none() {
                break;
            }
        }

        result
    }
}