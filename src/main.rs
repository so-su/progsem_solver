mod compare_nat_1;
mod compare_nat_2;
mod compare_nat_3;
mod nat;
mod utility;

fn main() {
    let deriv = compare_nat_3::CompareNat3::derive_u8(2, 5);
    println!("{}", utility::indent(deriv.to_string()));
}
