use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Instr {
    Inp(char),
    Add(char, [char; 3]),
    Mul(char, [char; 3]),
    Div(char, [char; 3]),
    Mod(char, [char; 3]),
    Eql(char, [char; 3]),
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
            println!("{}: {}", k, v.len());
            println!()
        }

        return 0;

        let mut last_input = 99999999999999;
        let mut input = 99999999999999;

        loop {
            let mut vars = HashMap::new();
            vars.insert('w', 0);
            vars.insert('x', 0);
            vars.insert('y', 0);
            vars.insert('z', 0);

            for &instr in &self.instructions {
                match instr {
                    Instr::Inp(a) => {
                        vars.entry(a).and_modify(|v| *v = input % 10);
                        input /= 10;
                    }
                    Instr::Add(a, b) => {
                        let true_b = get_number(&b, &vars);
                        vars.entry(a).and_modify(|v| *v += true_b);
                    }
                    Instr::Mul(a, b) => {
                        let true_b = get_number(&b, &vars);
                        vars.entry(a).and_modify(|v| *v *= true_b);
                    }
                    Instr::Div(a, b) => {
                        let true_b = get_number(&b, &vars);
                        vars.entry(a).and_modify(|v| *v /= true_b);
                    }
                    Instr::Mod(a, b) => {
                        let true_b = get_number(&b, &vars);
                        vars.entry(a).and_modify(|v| *v = *v % true_b);
                    }
                    Instr::Eql(a, b) => {
                        let true_b = get_number(&b, &vars);
                        vars.entry(a).and_modify(|v| *v += if *v == true_b { 1 } else { 0 });
                    }
                }
            }

            if *vars.get(&'z').unwrap() == 0 { return last_input as usize; }

            'a: loop {
                last_input -= 1;
                input = last_input.clone();

                for _ in 0..14 {
                    if input % 10 == 0 {
                        continue 'a;
                    }
                    input /= 10;
                }

                input = 0;
                for i in 0..14 {
                    input += (last_input / 10_isize.pow(i) % 10) * 10_isize.pow(13 - i);
                }
                if last_input % 1000000 == 111111 {
                    println!("{}", last_input);
                }
                break;
            }
        }
    }

    fn part2(&self) -> usize {
        0
    }
}

fn stringifier(instructions: &Vec<Instr>) -> HashMap<char, String> {
    let mut vars = HashMap::new();
    let mut input_counter = 0;
    vars.insert('w', String::from("0"));
    vars.insert('x', String::from("0"));
    vars.insert('y', String::from("0"));
    vars.insert('z', String::from("0"));
    let mut i = 0;

    for &instr in instructions {
        i += 1;
        println!("We are on line {}", i);
        match instr {
            Instr::Inp(a) => {
                vars.entry(a).and_modify(|v| *v = format!("I{}", input_counter));
                input_counter += 1;
            }
            Instr::Add(a, b) => {
                let true_b = get_str(&b, &vars);
                if true_b != "0" {
                    vars.entry(a).and_modify(|v| {
                        *v = if *v == "0" { true_b } else { format!("({})+({})", v, true_b) }
                    });
                }
            }
            Instr::Mul(a, b) => {
                let true_b = get_str(&b, &vars);
                if true_b == "0" {
                    vars.entry(a).and_modify(|v| *v = String::from("0"));
                } else if true_b != "1" {
                    vars.entry(a).and_modify(|v| {
                        *v = if *v == "0" { String::from("0") } else { format!("({})*({})", v, true_b) };
                    });
                }
            }
            Instr::Div(a, b) => {
                let true_b = get_str(&b, &vars);
                if true_b != "1" {
                    vars.entry(a).and_modify(|v| {
                        *v = if *v == "0" { String::from("0") } else { format!("({})/({})", v, true_b) }
                    });
                }
            }
            Instr::Mod(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| {
                    if *v == true_b || *v == "0" {
                        *v = String::from("0");
                    } else {
                        *v = format!("({})%({})", v, true_b);
                    }
                });
            }
            Instr::Eql(a, b) => {
                let true_b = get_str(&b, &vars);
                vars.entry(a).and_modify(|v| {
                    if *v == true_b {
                        *v = String::from("1");
                    } else {
                        *v = format!("({})==({})", v, true_b);
                    }
                });
            }
        }
    }

    vars
}

fn get_str(c: &[char; 3], vars: &HashMap<char, String>) -> String {
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
    if is_negative { (-result).to_string() } else { result.to_string() }
}

fn get_number(c: &[char; 3], vars: &HashMap<char, isize>) -> isize {
    if c[0].is_alphabetic() { return *vars.get(&c[0]).unwrap(); }

    let mut result = 0;
    let mut is_negative = false;

    for i in 0..3 {
        if c[i] == '-' {
            is_negative = true;
            continue;
        } else if c[i] as usize != 0  {
            result *= 10;
            result += c[i] as isize - 48;
        }
    }

    if is_negative { -result } else { result }
}