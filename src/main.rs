mod nat;
mod utility;

fn main() {
    let deriv = nat::NatDerivTimes::derive_u8(2, 2, 4);
    println!("{}", utility::indent(deriv.to_string()));
}
