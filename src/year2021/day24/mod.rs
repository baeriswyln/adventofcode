use std::borrow::Borrow;
use std::collections::HashMap;
use regex::Regex;
use evalexpr::*;

#[derive(Copy, Clone)]
enum Instr {
    Inp(char),
    Add(char, [char; 3]),
    Mul(char, [char; 3]),
    Div(char, [char; 3]),
    Mod(char, [char; 3]),
    Eql(char, [char; 3]),
}

#[derive(Clone)]
enum Value {
    Num(isize),
    Str(String),
}

impl Instr {
    fn compute(&self, v: &mut Value, other: &Value, re: &Regex) -> Value {
        match self {
            Instr::Inp(_) => Value::Str(String::from("I")),
            Instr::Add(_, _) => match v {
                Value::Num(na) => match other {
                    Value::Num(nb) => Value::Num(*na + *nb),
                    Value::Str(sb) => if *na == 0 { other.clone() } else { Value::Str(format!("{}+({})", na, sb)) }
                }
                Value::Str(sa) => match other {
                    Value::Num(nb) => if *nb == 0 { other.clone() } else { Value::Str(format!("{}+({})", nb, sa)) },
                    Value::Str(sb) => Value::Str(format!("({})+({})", sa, sb))
                }
            },
            Instr::Mul(_, _) => match v {
                Value::Num(na) => match other {
                    Value::Num(nb) => Value::Num(*na * *nb),
                    Value::Str(sb) => if *na == 0 { Value::Num(0) } else if *na == 1 { v.clone() } else { Value::Str(format!("{}*({})", na, sb)) }
                }
                Value::Str(sa) => match other {
                    Value::Num(nb) => if *nb == 0 { Value::Num(0) } else if *nb == 1 { v.clone() } else { Value::Str(format!("{}*({})", nb, sa)) },
                    Value::Str(sb) => Value::Str(format!("({})*({})", sa, sb))
                }
            },
            Instr::Div(_, _) => match v {
                Value::Num(na) => match other {
                    Value::Num(nb) => Value::Num(*na / *nb),
                    Value::Str(sb) => if *na == 0 { Value::Num(0) } else { Value::Str(format!("{}/({})", na, sb)) }
                }
                Value::Str(sa) => match other {
                    Value::Num(nb) => if *nb == 1 { v.clone() } else { Value::Str(format!("({})/{}", sa, nb)) },
                    Value::Str(sb) => Value::Str(format!("({})/({})", sa, sb))
                }
            },
            Instr::Mod(_, _) => match v {
                Value::Num(na) => match other {
                    Value::Num(nb) => Value::Num(*na % nb),
                    Value::Str(sb) => if *na == 0 { Value::Num(0) } else { Value::Str(format!("{}%({})", na, sb)) }
                }
                Value::Str(sa) => match other {
                    Value::Num(nb) => if *nb == 0 { Value::Num(0) } else if *nb == 1 { v.clone() } else { Value::Str(format!("({})%{}", sa, nb)) },
                    Value::Str(sb) => Value::Str(format!("({})%({})", sa, sb))
                }
            },
            Instr::Eql(_, _) => match v {
                Value::Num(na) => match other {
                    Value::Num(nb) => Value::Num(if *na == *nb { 1 } else { 0 }),
                    Value::Str(sb) => if (*na < 1 || *na > 9) && re.is_match(sb) { Value::Num(0) } else { Value::Str(format!("{}==({})", na, sb)) }
                }
                Value::Str(sa) => match other {
                    Value::Num(nb) => Value::Str(format!("{}==({})", nb, sa)),
                    Value::Str(sb) => if *sa == *sb {
                        Value::Num(0)
                    } else if re.is_match(sa) && re.is_match(sb) && !sa.contains("==") && !sb.contains("==") && !sa.contains("%") && !sb.contains("%") {
                        if eval_int(re.replace_all(sa, "1").borrow()).unwrap() > 9 { Value::Num(0) } else { Value::Str(format!("({})==({})", sa, sb)) }
                    } else { Value::Str(format!("({})==({})", sa, sb)) }
                },
            }
        }
    }
}

pub struct ALU {
    instructions: Vec<Instr>,
}

impl crate::Advent for ALU {
    fn new(data: &str) -> Self {
        ALU {
            instructions: data.lines().map(|l| {
                let mut content = l.split_whitespace();
                let instruction = content.next().unwrap();
                let a = content.next().unwrap().chars().next().unwrap();
                let mut b: [char; 3] = [0 as char; 3];

                if instruction != "inp" {
                    let mut i = 0;
                    for c in content.next().unwrap().chars() {
                        b[i] = c;
                        i += 1;
                    }
                }

                match instruction {
                    "add" => Instr::Add(a, b),
                    "mul" => Instr::Mul(a, b),
                    "div" => Instr::Div(a, b),
                    "mod" => Instr::Mod(a, b),
                    "eql" => Instr::Eql(a, b),
                    _ => Instr::Inp(a),
                }
            }).collect()
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 24 }

    fn part1(&self) -> usize {
        for (k, v) in stringifier(&self.instructions).iter() {
            match v {
                Value::Str(s) => println!("{}: {}", k, s.len()),
                Value::Num(n) => println!("{}: {}", k, n)
            }
        }

        0
    }

    fn part2(&self) -> usize {
        0
    }
}

fn stringifier(instructions: &Vec<Instr>) -> HashMap<char, Value> {
    let mut vars = HashMap::new();
    let re = Regex::new(r"I\d{2}").unwrap();
    let mut input_counter = 0;
    vars.insert('w', Value::Num(0));
    vars.insert('x', Value::Num(0));
    vars.insert('y', Value::Num(0));
    vars.insert('z', Value::Num(0));

    let mut i = 0;
    for &instr in instructions {
        i += 1;
        match instr {
            Instr::Inp(a) => {
                vars.entry(a).and_modify(|v| *v = Value::Str(format!("I{:0>2}", input_counter)));
                input_counter += 1;
            }
            Instr::Add(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| *v = instr.compute(v, &true_b, &re));
            }
            Instr::Mul(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| *v = instr.compute(v, &true_b, &re));
            }
            Instr::Div(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| *v = instr.compute(v, &true_b, &re));
            }
            Instr::Mod(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| *v = instr.compute(v, &true_b, &re));
            }
            Instr::Eql(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| *v = instr.compute(v, &true_b, &re));
            }
        }
        println!("instr {}", i);
        for (k, v) in vars.iter() {
            match v {
                Value::Str(s) => if s.len()>200 {
                    println!("{}: {}", k, s.len())
                } else {
                    println!("{}: {}", k, s)
                },
                Value::Num(n) => println!("{}: {}", k, n)
            }
        }
        println!();
    }

    vars
}

fn get_str(c: &[char; 3], vars: &HashMap<char, Value>) -> Value {
    if c[0].is_alphabetic() { return vars.get(&c[0]).unwrap().clone(); }

    let mut result = 0;
    let mut is_negative = false;

    for i in 0..3 {
        if c[i] == '-' {
            is_negative = true;
            continue;
        } else if c[i] as usize != 0 {
            result *= 10;
            result += c[i] as isize - 48;
        }
    }
    if is_negative { Value::Num(-result) } else { Value::Num(result) }
}