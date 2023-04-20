mod compare_nat_1;
mod nat;
mod utility;

fn main() {
    let deriv = compare_nat_1::CompareNat1::derive_u8(2, 5);
    println!("{}", utility::indent(deriv.to_string()));
}
