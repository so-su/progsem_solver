use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Nat {
    Z,
    S(Box<Nat>),
}

impl Nat {
    pub fn new(n: u8) -> Nat {
        if n == 0 {
            Nat::Z
        } else {
            Nat::S(Box::new(Nat::new(n - 1)))
        }
    }
}

// Natのたし算
impl Add for Nat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        use Nat::*;
        match self {
            Z => rhs,
            S(n) => n.add(S(Box::new(rhs))),
        }
    }
}

// Natのかけ算
impl Mul for Nat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        use Nat::*;
        match self {
            Z => Z,
            S(n) => rhs.clone().add(n.mul(rhs)),
        }
    }
}

impl Display for Nat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum NatDerivPlus {
    PZero {
        n: Nat,
    },
    PSucc {
        n1: Nat,
        n2: Nat,
        n: Nat,
        deriv1: Box<NatDerivPlus>,
    },
}

impl NatDerivPlus {
    // 導出を構成する
    pub fn derive(n1: Nat, n2: Nat, n3: Nat) -> NatDerivPlus {
        use Nat::*;
        match (n1, n2, n3) {
            (Z, n2, n3) if n2 == n3 => Self::PZero { n: n2 },
            (S(n1), n2, S(n)) => Self::PSucc {
                n1: *n1.clone(),
                n2: n2.clone(),
                n: *n.clone(),
                deriv1: Box::new(NatDerivPlus::derive(*n1, n2, *n)),
            },
            _ => panic!(),
        }
    }

    // u8から導出を構成する
    pub fn derive_u8(n1: u8, n2: u8, n3: u8) -> NatDerivPlus {
        Self::derive(Nat::new(n1), Nat::new(n2), Nat::new(n3))
    }
}

impl Display for NatDerivPlus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Nat::*;
        match self {
            Self::PZero { n } => {
                write!(f, "Z plus {} is {} by P-Zero{{}}", n, n)?;
            }
            Self::PSucc { n1, n2, n, deriv1 } => {
                write!(
                    f,
                    "{} plus {} is {} by P-Succ{{{}}}",
                    S(Box::new(n1.clone())),
                    n2,
                    S(Box::new(n.clone())),
                    deriv1
                )?;
            }
        }
        Ok(())
    }
}

pub enum NatDerivTimes {
    TZero {
        n: Nat,
    },
    TSucc {
        n1: Nat,
        n2: Nat,
        n4: Nat,
        deriv1: Box<NatDerivTimes>,
        deriv2: NatDerivPlus,
    },
}

impl NatDerivTimes {
    // 導出を構成する
    pub fn derive(n1: Nat, n2: Nat, n3: Nat) -> NatDerivTimes {
        use Nat::*;
        match (n1, n2, n3) {
            (Z, n2, Z) => Self::TZero { n: n2 },
            (S(n1), n2, n4) => {
                let n3 = *n1.clone() * n2.clone();
                Self::TSucc {
                    n1: *n1.clone(),
                    n2: n2.clone(),
                    n4: n4.clone(),
                    deriv1: Box::new(NatDerivTimes::derive(*n1, n2.clone(), n3.clone())),
                    deriv2: NatDerivPlus::derive(n2, n3, n4),
                }
            }
            _ => panic!(),
        }
    }

    // u8から導出を構成する
    pub fn derive_u8(n1: u8, n2: u8, n3: u8) -> NatDerivTimes {
        Self::derive(Nat::new(n1), Nat::new(n2), Nat::new(n3))
    }
}

impl Display for NatDerivTimes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Nat::*;
        match self {
            Self::TZero { n } => {
                write!(f, "Z times {} is Z by T-Zero{{}}", n)?;
            }
            Self::TSucc {
                n1,
                n2,
                n4,
                deriv1,
                deriv2,
            } => {
                write!(
                    f,
                    "{} times {} is {} by T-Succ{{{};{}}}",
                    S(Box::new(n1.clone())),
                    n2,
                    n4,
                    deriv1,
                    deriv2
                )?;
            }
        }
        Ok(())
    }
}
