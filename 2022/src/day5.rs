use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cargo {
    towers: Vec<Vec<char>>,
}

impl Cargo {
    pub fn apply(&mut self, command: Command) {
        for _ in 0..command.n {
            let x = self.towers[command.from].pop().unwrap();
            self.towers[command.to].push(x);
        }
    }

    pub fn apply_multimove(&mut self, command: Command) {
        let mut buf = Vec::new();
        for _ in 0..command.n {
            let c = self.towers[command.from].pop().unwrap();
            buf.push(c);
        }
        while let Some(c) = buf.pop() {
            self.towers[command.to].push(c);
        }
    }

    pub fn apply_all(&mut self, commands: Commands) {
        for command in commands.commands {
            self.apply(command);
        }
    }

    pub fn apply_all_multimove(&mut self, commands: Commands) {
        for command in commands.commands {
            self.apply_multimove(command);
        }
    }

    pub fn top_crates(&self) -> String {
        self.towers
            .iter()
            .map(|tower| tower.last().unwrap())
            .copied()
            .collect()
    }
}

impl FromStr for Cargo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
            .rev()
            .skip(1)
            .collect::<Vec<_>>();
        let mut towers = vec![vec![]; rows[0].len()];
        for row in rows.into_iter() {
            for (i, c) in row.into_iter().enumerate() {
                if c != ' ' {
                    towers[i].push(c)
                }
            }
        }
        Ok(Cargo { towers })
    }
}

pub fn split_at_nl(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Commands {
    commands: Vec<Command>,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let commands = s
            .lines()
            .map(|line| Command::from_str(line).unwrap())
            .collect();
        Ok(Commands { commands })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    n: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        Ok(Command {
            n: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1,
        })
    }
}

pub fn top_crates_after_commands(input: &str) -> String {
    let (cargo_str, commands_str) = split_at_nl(input);
    let mut cargo = Cargo::from_str(cargo_str).unwrap();
    let commands = Commands::from_str(commands_str).unwrap();
    cargo.apply_all(commands);
    cargo.top_crates()
}

pub fn top_crates_after_commands_multimove(input: &str) -> String {
    let (cargo_str, commands_str) = split_at_nl(input);
    let mut cargo = Cargo::from_str(cargo_str).unwrap();
    let commands = Commands::from_str(commands_str).unwrap();
    cargo.apply_all_multimove(commands);
    cargo.top_crates()
}

#[cfg(test)]
mod tests {
    use crate::{content, day5::top_crates_after_commands_multimove};

    use super::{top_crates_after_commands, Cargo, Command, Commands};
    use std::str::FromStr;

    #[test]
    fn test_move() {
        let mut cargo = Cargo {
            towers: vec![vec!['a', 'b', 'c'], vec![]],
        };
        cargo.apply(Command {
            n: 2,
            from: 0,
            to: 1,
        });
        assert_eq!(
            cargo,
            Cargo {
                towers: vec![vec!['a'], vec!['c', 'b']]
            }
        )
    }

    #[test]
    fn parse_test() {
        let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3"#;
        let cargo = Cargo::from_str(input).unwrap();
        assert_eq!(
            cargo,
            Cargo {
                towers: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
            }
        )
    }

    #[test]
    fn parse_commands() {
        let commands = r#"move 1 from 2 to 3
        move 4 from 5 to 6"#;

        let commands = Commands::from_str(commands).unwrap();
        assert_eq!(
            commands,
            Commands {
                commands: vec![
                    Command {
                        n: 1,
                        from: 1,
                        to: 2
                    },
                    Command {
                        n: 4,
                        from: 4,
                        to: 5
                    }
                ]
            }
        )
    }

    #[test]
    fn example() {
        let example = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let tops = top_crates_after_commands(example);
        assert_eq!(tops, "CMZ");
    }

    #[test]
    fn part1() {
        let data = content("./data/day5.txt");
        let tops = top_crates_after_commands(&data);
        assert_eq!(tops, "QNHWJVJZW");
    }

    #[test]
    fn part2() {
        let data = content("./data/day5.txt");
        let tops = top_crates_after_commands_multimove(&data);
        assert_eq!(tops, "BPCZJLFJW");
    }
}
