use std::str::FromStr;

pub const AND: &str = "AND";
pub const OR: &str = "OR";
pub const NAND: &str = "NAND";
pub const NOR: &str = "NOR";
pub const XOR: &str = "XOR";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    And = 0,
    Or = 1,
    Nand = 2,
    Nor = 3,
    Xor = 4,
}

impl From<u32> for Gate {
    fn from(num: u32) -> Self {
        match num {
            0 => Self::And,
            1 => Self::Or,
            2 => Self::Nand,
            3 => Self::Nor,
            4 => Self::Xor,
            _ => panic!("invalid number"),
        }
    }
}

impl FromStr for Gate {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let gate = match input {
            AND => Self::And,
            OR => Self::Or,
            NAND => Self::Nand,
            NOR => Self::Nor,
            XOR => Self::Xor,
            _ => anyhow::bail!("invalid input {}", input),
        };
        Ok(gate)
    }
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed = match self {
            Self::And => AND,
            Self::Or => OR,
            Self::Nand => NAND,
            Self::Nor => NOR,
            Self::Xor => XOR,
        };
        write!(f, "{}", displayed)
    }
}

#[test]
fn boolean_gate_to_and_from_string() {
    let mut gate = Gate::from_str(AND).unwrap();
    assert_eq!(gate.to_string(), AND);

    gate = Gate::from_str(OR).unwrap();
    assert_eq!(gate.to_string(), OR);

    gate = Gate::from_str(NAND).unwrap();
    assert_eq!(gate.to_string(), NAND);

    gate = Gate::from_str(NOR).unwrap();
    assert_eq!(gate.to_string(), NOR);

    gate = Gate::from_str(XOR).unwrap();
    assert_eq!(gate.to_string(), XOR);

    let other = Gate::from_str(&gate.to_string()).unwrap();
    assert_eq!(gate, other);
    assert!(Gate::from_str("invalid").is_err());
    assert!(Gate::from_str("123").is_err());
}
