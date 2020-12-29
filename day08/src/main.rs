use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Instr {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let subs: Vec<&str> = s.splitn(2, ' ').collect();
        let arg = subs[1]
            .parse::<i32>()
            .map_err(|_| "Could not parse integer argument.".to_string())?;

        match subs[0] {
            "nop" => Ok(Self::Nop(arg)),
            "acc" => Ok(Self::Acc(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            other => Err(format!("Invalid opcode \"{}\".", other)),
        }
    }
}

type Prog = Vec<Instr>;

struct ProgState<'a> {
    map: HashMap<i32, &'a Instr>,
    pc: i32,
    acc: i32,
}

impl<'a> From<&'a Prog> for ProgState<'a> {
    fn from(prog: &'a Prog) -> Self {
        Self {
            pc: 0,
            acc: 0,
            map: (0i32..).zip(prog).collect(),
        }
    }
}

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let prog: Prog = contents
        .lines()
        .map(|l| l.parse::<Instr>())
        .collect::<Result<_, _>>()?;

    let res = run_prog(&prog);
    println!("Part 1: {}.", res);
    let res = try_run(&prog);
    println!("Part 2: {}.", res.expect("No result found"));

    Ok(())
}

fn step(state: &mut ProgState, instr: &Instr) {
    match instr {
        Instr::Nop(_) => state.pc += 1,
        Instr::Acc(arg) => {
            state.acc += arg;
            state.pc += 1
        }
        Instr::Jmp(arg) => state.pc += arg,
    };
}

fn run_prog(prog: &Prog) -> i32 {
    let mut state = ProgState::from(prog);

    while let Some(instr) = state.map.remove(&state.pc) {
        step(&mut state, &instr);
    }

    state.acc
}

fn run_prog2(prog: &Prog) -> Option<i32> {
    let mut state = ProgState::from(prog);

    loop {
        if state.pc as usize == prog.len() {
            return Some(state.acc);
        } else if let Some(instr) = state.map.remove(&state.pc) {
            step(&mut state, &instr);
        } else {
            return None;
        }
    }
}

fn try_run(prog: &Prog) -> Option<i32> {
    for i in 0..prog.len() {
        let mut copy = prog.to_vec();

        match prog[i] {
            Instr::Nop(arg) => copy[i] = Instr::Jmp(arg),
            Instr::Jmp(arg) => copy[i] = Instr::Nop(arg),
            _ => continue,
        }

        if let Some(res) = run_prog2(&copy) {
            return Some(res);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instr() -> Result<(), String> {
        assert_eq!("nop +13".parse::<Instr>()?, Instr::Nop(13));
        assert_eq!("acc +13".parse::<Instr>()?, Instr::Acc(13));
        assert_eq!("acc -13".parse::<Instr>()?, Instr::Acc(-13));
        assert_eq!("jmp -13".parse::<Instr>()?, Instr::Jmp(-13));

        Ok(())
    }

    fn get_prog() -> Prog {
        Vec::from([
            Instr::Nop(0),
            Instr::Acc(1),
            Instr::Jmp(4),
            Instr::Acc(3),
            Instr::Jmp(-3),
            Instr::Acc(-99),
            Instr::Acc(1),
            Instr::Jmp(-4),
            Instr::Acc(6),
        ])
    }

    #[test]
    fn test_run_prog() {
        assert_eq!(run_prog(&get_prog()), 5);
    }

    #[test]
    fn test_run_prog2() {
        let mut prog = get_prog();
        assert_eq!(run_prog2(&prog), None);
        prog[7] = Instr::Nop(2);
        assert_eq!(run_prog2(&prog), Some(8));
    }

    #[test]
    fn test_try_run() {
        assert_eq!(try_run(&get_prog()), Some(8));
    }
}
