use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Address {
    Exact(u16),
    Label(String),
}

impl Address {
    fn into_exact(&self, labels: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Self::Exact(n) => Some(*n),
            Self::Label(l) => labels.get(l).map(|x| *x),
        }
    }
}

impl FromStr for Address {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(n) = s.parse::<u16>() {
            Self::Exact(n)
        } else {
            Self::Label(s.to_string())
        })
    }
}

#[derive(Debug)]
enum Instructions {
    Add(Address),
    Sub(Address),
    Sta(Address),
    Lda(Address),
    Bra(Address),
    Brz(Address),
    Brp(Address),
    Inp,
    Out,
    Hlt,
    Dat(Option<i16>),
}

impl Instructions {
    fn parse(mnenomic: &str, argument: Option<&str>) -> Option<Self> {
        match mnenomic.to_ascii_uppercase().as_str() {
            "ADD" => Some(Self::Add(Address::from_str(argument?).ok()?)),
            "SUB" => Some(Self::Sub(Address::from_str(argument?).ok()?)),
            "STA" => Some(Self::Sta(Address::from_str(argument?).ok()?)),
            "LDA" => Some(Self::Lda(Address::from_str(argument?).ok()?)),
            "BRA" => Some(Self::Bra(Address::from_str(argument?).ok()?)),
            "BRZ" => Some(Self::Brz(Address::from_str(argument?).ok()?)),
            "BRP" => Some(Self::Brp(Address::from_str(argument?).ok()?)),
            "INP" => Some(Self::Inp),
            "OUT" => Some(Self::Out),
            "HLT" => Some(Self::Hlt),
            "DAT" => Some(Self::Dat(argument.and_then(|a| a.parse::<i16>().ok()))),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct IR {
    labels: HashMap<String, u16>,
    instructions: Vec<Instructions>,
}

impl IR {
    pub fn parse_lines(input: &str) -> Self {
        let mut labels = HashMap::new();
        let mut instructions = vec![];

        for (i, line) in input.lines().enumerate() {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            instructions.push(if parts.len() == 3 {
                labels.insert(parts[0].to_string(), i as u16);

                Instructions::parse(parts[1], parts.get(2).map(|x| *x)).expect(&format!(
                    "Failed to read instruction: {:?} {:?}",
                    parts[1],
                    parts.get(2)
                ))
            } else {
                match Instructions::parse(parts[0], parts.get(1).map(|x| *x)) {
                    Some(i) => i,
                    None => {
                        labels.insert(parts[0].to_string(), i as u16);

                        Instructions::parse(parts[1], None).expect(&format!(
                            "Failed to read instruction: {:?} {:?}",
                            parts[0],
                            parts.get(1)
                        ))
                    }
                }
            });
        }

        Self {
            labels,
            instructions,
        }
    }

    pub fn assemble(self) -> [i16; 100] {
        let mut memory = [0; 100];

        for (i, instruction) in self.instructions.into_iter().enumerate() {
            memory[i] = match instruction {
                Instructions::Add(addr) => {
                    100 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }
                Instructions::Sub(addr) => {
                    200 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }
                Instructions::Sta(addr) => {
                    300 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }
                Instructions::Lda(addr) => {
                    500 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }
                Instructions::Bra(addr) => {
                    600 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }

                Instructions::Brz(addr) => {
                    700 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }

                Instructions::Brp(addr) => {
                    800 + addr
                        .into_exact(&self.labels)
                        .expect("Failed to expand label") as i16
                }

                Instructions::Inp => 901,
                Instructions::Out => 902,
                Instructions::Hlt => 000,
                Instructions::Dat(n) => n.unwrap_or(0),
            };
        }

        memory
    }
}
