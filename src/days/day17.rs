use std::fmt;

#[derive(Debug)]
struct Program {
    A: u64,
    B: u64,
    C: u64,

    pc: usize,
    instructions: Vec<u64>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "R| A:{}, B:{}, C:{}\npc: {}",
            self.A, self.B, self.C, self.pc
        )
    }
}

impl Program {
    fn combo_op(&self, operand_val: u64) -> u64 {
        match operand_val {
            0..=3 => operand_val,
            4 => self.A,
            5 => self.B,
            6 => self.C,
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
        (self.A as f64 / (2_u64.pow(self.combo_op(operand) as u32)) as f64).floor() as u64
    }

    fn adv(&mut self, operand: u64) -> Option<u64> {
        self.A = self.dv(operand);
        None
    }

    fn bxl(&mut self, operand: u64) -> Option<u64> {
        self.B ^= operand;
        None
    }

    fn bst(&mut self, operand: u64) -> Option<u64> {
        self.B = self.combo_op(operand) % 8;
        None
    }

    fn jnz(&mut self, operand: u64) -> Option<u64> {
        if self.A != 0 {
            self.pc = operand as usize;
        }
        None
    }

    fn bxc(&mut self, operand: u64) -> Option<u64> {
        self.B = self.B ^ self.C;
        None
    }

    fn out(&mut self, operand: u64) -> Option<u64> {
        Some(self.combo_op(operand) % 8)
    }

    fn bdv(&mut self, operand: u64) -> Option<u64> {
        self.B = self.dv(operand);
        None
    }

    fn cdv(&mut self, operand: u64) -> Option<u64> {
        self.C = self.dv(operand);
        None
    }

    fn run(&mut self) -> String {
        let mut output: Vec<u64> = Vec::new();
        while self.pc < self.instructions.len() {
            println!("{}", self);
            println!("{:?}", output);
            let (op, operand) = (self.instructions[self.pc], self.instructions[self.pc + 1]);
            if let Some(o) = self.run_op(op, operand) {
                output.push(o);
            }
            if op != 3 || self.A == 0 {
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
        A: d_[0],
        B: d_[1],
        C: d_[2],
        pc: 0,
        instructions: program,
    };
    println!("{}", program.run());

    0
}

pub fn part2(input: String) -> u64 {
    0
}
