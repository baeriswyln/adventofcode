#[derive(PartialEq)]
enum ElementType {
    File,
    Folder,
}

struct Element {
    name: String,
    element_type: ElementType,
    children: Vec<Element>,
    size: usize,
}

impl Element {
    fn new_folder(name: String) -> Self {
        Element {
            name,
            element_type: ElementType::Folder,
            children: vec![],
            size: 0,
        }
    }

    fn new_file(name: String, size: usize) -> Self {
        Element {
            name,
            element_type: ElementType::File,
            children: vec![],
            size,
        }
    }

    fn size(&self) -> usize {
        match self.element_type {
            ElementType::File => self.size,
            ElementType::Folder => self.children.iter().map(|e| e.size()).sum()
        }
    }

    fn sum(&self) -> usize {
        match self.element_type {
            ElementType::Folder => {
                let mut result = 0;
                println!("{}", self.size());
                if self.size() <= 100000 {
                    result += self.size();
                }

                for child in &self.children {
                    result += child.sum();
                }

                result
            }
            ElementType::File => 0
        }
    }

    fn smallest_to_delete(&self, min: usize, current_min: usize) -> Option<usize> {
        let mut result = current_min;
        let my_size = self.size();
        if my_size >= min && self.element_type == ElementType::Folder{
            if my_size < result {
                result = my_size
            }

            for child in &self.children {
                match child.smallest_to_delete(min, result) {
                    Option::Some(value) => if value < result { result = value },
                    Option::None => ()
                };
            }
            return Option::Some(result);
        }

        Option::None
    }

    fn add(&mut self, item: Element, name: &Vec<String>, index: usize) {
        if index == name.len() {
            self.children.push(item)
        } else {
            for i in 0..self.children.len() {
                if self.children.get(i).unwrap().name == name[index] {
                    self.children.get_mut(i).unwrap().add(item, name, index + 1);
                    return;
                }
            }
            println!("folder {:?} not found in {} at index {}", name, self.name, index)
        }
    }
}

pub struct OutOfSpace {
    root: Element,
}

impl crate::Advent for OutOfSpace {
    fn new(data: &str) -> Self {
        let mut root = Element::new_folder(String::from("/"));
        let mut current_path = vec![];

        data.lines().for_each(|line| {
            let mut line_iter = line.split(" ");

            match line_iter.next().unwrap() {
                "$" => match line_iter.next().unwrap() {
                    "cd" => match line_iter.next().unwrap() {
                        ".." => { current_path.pop(); }
                        "/" => current_path = vec![],
                        name => current_path.push(String::from(name))
                    },
                    _ => ()
                },
                "dir" => root.add(
                    Element::new_folder(String::from(line_iter.next().unwrap())),
                    &current_path,
                    0,
                ),
                size => root.add(
                    Element::new_file(
                        String::from(line_iter.next().unwrap()),
                        size.parse().unwrap(),
                    ),
                    &current_path,
                    0,
                )
            };
        });

        OutOfSpace {
            root
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        7
    }

    fn part1(&self) -> usize {
        self.root.sum()
    }

    fn part2(&self) -> usize {
        let min_folder = self.root.size();
        let available = 70000000 - min_folder;
        let min = 30000000 - available;

        self.root.smallest_to_delete(min, min_folder).unwrap()
    }
}
