use std::fs;

type Crate = char;
type Stack = Vec<Crate>;
type Stacks = Vec<Stack>;
type Instruction = (usize, usize, usize);
type Instructions = Vec<Instruction>;

fn parse_stacks(s: &str) -> Stacks {
    let mut lines = s.lines().rev();

    let mut columns: Vec<usize> = Default::default();
    let mut stacks: Stacks = Default::default();

    for (i, c) in lines.next().unwrap().chars().enumerate() {
        if !c.is_whitespace() {
            columns.push(i);
            stacks.push(Default::default());
        }
    }

    for line in lines {
        for (i, &column) in columns.iter().enumerate() {
            let c = line.chars().nth(column).unwrap();
            if !c.is_whitespace() {
                stacks[i].push(c);
            }
        }
    }

    stacks
}

fn parse_instructions(s: &str) -> Instructions {
    s.lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();

            let count: usize = tokens.nth(1).unwrap().parse().unwrap();
            let from: usize = tokens.nth(1).unwrap().parse().unwrap();
            let to: usize = tokens.nth(1).unwrap().parse().unwrap();

            (count, from - 1, to - 1)
        })
        .collect()
}

fn rearrange_stacks(stacks: &mut Stacks, instructions: &Instructions) {
    for (count, from, to) in instructions.iter() {
        for _ in 0..*count {
            let c = stacks[*from].pop().unwrap();
            stacks[*to].push(c);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut chunks = input.split("\n\n");

    let mut stacks = parse_stacks(chunks.next().unwrap());
    let instructions = parse_instructions(chunks.next().unwrap());

    rearrange_stacks(&mut stacks, &instructions);

    let message: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    dbg!(message);
}
