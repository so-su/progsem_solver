use crate::nat::*;

#[derive(Debug)]
pub enum CompareNat3 {
    LSucc {
        n: Nat,
    },
    LSuccR {
        n1: Nat,
        n2: Nat,
        deriv1: Box<CompareNat3>,
    },
}

impl CompareNat3 {
    // 導出を構成する
    pub fn derive(n1: Nat, n2: Nat) -> CompareNat3 {
        use Nat::*;
        match (n1, n2) {
            (n, S(n_)) if n == *n_ => Self::LSucc { n: n },
            (n1, S(n2)) => Self::LSuccR {
                n1: n1.clone(),
                n2: *n2.clone(),
                deriv1: Box::new(CompareNat3::derive(n1, *n2)),
            },
            _ => panic!(),
        }
    }

    // u8から導出を構成する
    pub fn derive_u8(n1: u8, n2: u8) -> CompareNat3 {
        Self::derive(Nat::new(n1), Nat::new(n2))
    }
}

impl std::fmt::Display for CompareNat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Nat::*;
        match self {
            Self::LSucc { n } => {
                write!(
                    f,
                    "{} is less than {}  by L-Succ{{}}",
                    n,
                    S(Box::new(n.clone()))
                )?;
            }
            Self::LSuccR { n1, n2, deriv1 } => {
                write!(
                    f,
                    "{} is less than {} by L-SuccR{{{}}}",
                    n1,
                    S(Box::new(n2.clone())),
                    deriv1,
                )?;
            }
        }
        Ok(())
    }
}
