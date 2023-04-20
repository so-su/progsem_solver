mod compare_nat_1;
mod compare_nat_2;
mod nat;
mod utility;

fn main() {
    let deriv = compare_nat_2::CompareNat2::derive_u8(2, 3);
    println!("{}", utility::indent(deriv.to_string()));
}
