use crate::nat::*;

#[derive(Debug)]
pub enum CompareNat1 {
    LSucc {
        n: Nat,
    },
    LTrans {
        n1: Nat,
        n3: Nat,
        deriv1: Box<CompareNat1>,
        deriv2: Box<CompareNat1>,
    },
}

impl CompareNat1 {
    // 導出を構成する
    pub fn derive(n1: Nat, n2: Nat) -> CompareNat1 {
        use Nat::*;
        match (n1, n2) {
            (_, Z) => panic!(),
            (n, S(n_)) if n == *n_ => Self::LSucc { n: n },
            (n1, n3) => {
                let n2 = S(Box::new(n1.clone()));
                Self::LTrans {
                    n1: n1.clone(),
                    n3: n3.clone(),
                    deriv1: Box::new(CompareNat1::derive(n1, n2.clone())),
                    deriv2: Box::new(CompareNat1::derive(n2, n3)),
                }
            }
        }
    }

    // u8から導出を構成する
    pub fn derive_u8(n1: u8, n2: u8) -> CompareNat1 {
        Self::derive(Nat::new(n1), Nat::new(n2))
    }
}

impl std::fmt::Display for CompareNat1 {
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
            Self::LTrans {
                n1,
                n3,
                deriv1,
                deriv2,
            } => {
                write!(
                    f,
                    "{} is less than {} by L-Trans{{{};{}}}",
                    n1, n3, deriv1, deriv2
                )?;
            }
        }
        Ok(())
    }
}
