use crate::nat::*;

#[derive(Debug)]
pub enum CompareNat2 {
    LZero {
        n: Nat,
    },
    LSuccSucc {
        n1: Nat,
        n2: Nat,
        deriv1: Box<CompareNat2>,
    },
}

impl CompareNat2 {
    // 導出を構成する
    pub fn derive(n1: Nat, n2: Nat) -> CompareNat2 {
        use Nat::*;
        match (n1, n2) {
            (Z, n) => Self::LZero { n: n },
            (S(n1), S(n2)) => Self::LSuccSucc {
                n1: *n1.clone(),
                n2: *n2.clone(),
                deriv1: Box::new(CompareNat2::derive(*n1, *n2)),
            },
            _ => panic!(),
        }
    }

    // u8から導出を構成する
    pub fn derive_u8(n1: u8, n2: u8) -> CompareNat2 {
        Self::derive(Nat::new(n1), Nat::new(n2))
    }
}

impl std::fmt::Display for CompareNat2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Nat::*;
        match self {
            Self::LZero { n } => {
                write!(f, "Z is less than {}  by L-Zero{{}}", Box::new(n))?;
            }
            Self::LSuccSucc { n1, n2, deriv1 } => {
                write!(
                    f,
                    "{} is less than {} by L-SuccSucc{{{}}}",
                    S(Box::new(n1.clone())),
                    S(Box::new(n2.clone())),
                    deriv1,
                )?;
            }
        }
        Ok(())
    }
}
