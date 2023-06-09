mod compare_nat_1;
mod compare_nat_2;
mod compare_nat_3;
mod nat;
mod utility;

fn main() {
    let deriv = nat::NatDerivPlus::derive_u8(2, 3, 5);
    println!("{}", utility::indent(deriv.to_string()));

    let deriv = nat::NatDerivTimes::derive_u8(2, 3, 6);
    println!("{}", utility::indent(deriv.to_string()));
}
