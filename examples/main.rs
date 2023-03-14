extern crate morello_capability_rs;

fn main() {
    let mut i : i32 = 0;
    let mut y : i64 = 0;
    let addr1 = morello_capability_rs::get_address(&mut i);
    let addr2 = morello_capability_rs::get_address(&mut y);
    println!("0x{:x}", addr1);
    println!("0x{:x}", addr2);
}
