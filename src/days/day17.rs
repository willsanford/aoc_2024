use std::fmt;

#[derive(Debug, Clone)]
struct Program {
    a: u64,
    b: u64,
    c: u64,

    pc: usize,
    instructions: Vec<u64>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "R| A:{}, B:{}, C:{}\npc: {}",
            self.a, self.b, self.c, self.pc
        )
    }
}

impl Program {
    fn combo_op(&self, operand_val: u64) -> u64 {
        match operand_val {
            0..=3 => operand_val,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => {
                println!("Something has gone horribly wrong please stop");
                0
            }
        }
    }

    fn run_op(&mut self, op: u64, operand: u64) -> Option<u64> {
        match op {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => {
                println!("Something has gone horribly wrong. Please debug");
                None
            }
        }
    }

    fn dv(&mut self, operand: u64) -> u64 {
        (self.a as f64 / (2_u64.pow(self.combo_op(operand) as u32)) as f64).floor() as u64
    }

    fn adv(&mut self, operand: u64) -> Option<u64> {
        self.a = self.dv(operand);
        None
    }

    fn bxl(&mut self, operand: u64) -> Option<u64> {
        self.b ^= operand;
        None
    }

    fn bst(&mut self, operand: u64) -> Option<u64> {
        self.b = self.combo_op(operand) % 8;
        None
    }

    fn jnz(&mut self, operand: u64) -> Option<u64> {
        if self.a != 0 {
            self.pc = operand as usize;
        }
        None
    }

    fn bxc(&mut self, operand: u64) -> Option<u64> {
        self.b = self.b ^ self.c;
        None
    }

    fn out(&mut self, operand: u64) -> Option<u64> {
        Some(self.combo_op(operand) % 8)
    }

    fn bdv(&mut self, operand: u64) -> Option<u64> {
        self.b = self.dv(operand);
        None
    }

    fn cdv(&mut self, operand: u64) -> Option<u64> {
        self.c = self.dv(operand);
        None
    }

    fn run(&mut self) -> String {
        let mut output: Vec<u64> = Vec::new();
        while self.pc < self.instructions.len() {
            let (op, operand) = (self.instructions[self.pc], self.instructions[self.pc + 1]);
            if let Some(o) = self.run_op(op, operand) {
                output.push(o);
            }
            if op != 3 || self.a == 0 {
                self.pc += 2;
            }
        }
        let mut out: String = String::new();
        for v in output {
            out.push_str(&v.to_string());
            out.push(',')
        }
        out
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn part1(input: String) -> u64 {
    let (d, p) = input.split_once("\n\n").unwrap();

    let mut d_: Vec<u64> = d
        .lines()
        .filter_map(|line| {
            let (_, n) = line.split_once(": ").unwrap();
            n.parse::<u64>().ok()
        })
        .collect();

    let (_, p_) = p.split_once(": ").unwrap();
    let program: Vec<u64> = p_
        .split(',')
        .filter_map(|v| remove_whitespace(v).parse::<u64>().ok())
        .collect();

    let mut program = Program {
        a: d_[0],
        b: d_[1],
        c: d_[2],
        pc: 0,
        instructions: program,
    };
    println!("{}", program.run());

    0
}

fn eval_full(seed: u64) {
    let mut a = seed;
    let mut b = 0;
    let mut c = 0;
    for _ in 0..16 {
        b = (a & 7u64) ^ 5_u64;
        c = a >> b;
        b = b ^ 6_u64;
        b = b ^ c;
        print!("{},", b & 7);
        a = a >> 3_u64;
    }
    println!("");
}

fn eval(a: u64) -> u64 {
    let b = (a & 7u64) ^ 5_u64;
    (b ^ (a >> b) ^ 6_u64) & 7_u64
}

fn recurse(pre_a: u64, out: &Vec<u64>, target: &Vec<u64>) -> Vec<Vec<u64>> {
    if out.len() == target.len() {
        return vec![out.to_vec()];
    }
    (0_u64..(1 << 3)).map(|i| pre_a + (i << 7)).filter_map(|a| {
        if eval(a) == target[out.len()] {
            let mut new_out: Vec<u64> = out.clone();
            new_out.push(if new_out.len() == target.len() - 1 {
                a
            } else {
                a & 7
            });
            Some(recurse(a >> 3, &new_out, target))
        } else {
            None
        }
    }).flatten().collect()
}

pub fn part2(input: String) -> u64 {
    let program: Vec<u64> = input
        .split_once("\n\n")
        .unwrap()
        .1
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .filter_map(|v| remove_whitespace(v).parse::<u64>().ok())
        .collect();

    let values: Vec<_> = (0..(1 << 10))
        .filter(|i| eval(*i) == program[0])
        .map(|seed| {
            let out = vec![seed & 7];
            recurse(seed >> 3, &out, &program)
        }).flatten().map(|values| {
        values.iter()
            .enumerate()
            .fold(0, |acc, (i, e)| acc + (e << (i * 3)))
    }).collect();
    *values.iter().min().unwrap()
    // println!("{:?}", final_solution);
    // println!("{:?}", eval_full(final_solution));
}
/*
fn recurse(pre_a: u64, out: &Vec<u64>, target: &Vec<u64>) -> Option<Vec<u64>> {
    if out.len() == target.len() {
        return Some(out.to_vec());
    }
    (0_u64..(1 << 3)).map(|i| pre_a + (i << 7)).find_map(|a| {
        if eval(a) == target[out.len()] {
            let mut new_out: Vec<u64> = out.clone();
            new_out.push(if new_out.len() == target.len() - 1 {
                a
            } else {
                a & 7
            });
            recurse(a >> 3, &new_out, target)
        } else {
            None
        }
    })
}

pub fn part2(input: String) -> u64 {
    let program: Vec<u64> = input
        .split_once("\n\n")
        .unwrap()
        .1
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .filter_map(|v| remove_whitespace(v).parse::<u64>().ok())
        .collect();

    (0..(1 << 10))
        .filter(|i| eval(*i) == program[0])
        .find_map(|seed| {
            let out = vec![seed & 7];
            recurse(seed >> 3, &out, &program)
        })
        .unwrap()
        .iter()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + (e << (i * 3)))

    // println!("{:?}", final_solution);
    // println!("{:?}", eval_full(final_solution));
}
*/
