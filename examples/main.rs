use std::mem::size_of;

use morello::capability::*;

fn main() {
    let arr : [i64; 4] = [ 0, 42, 0, 0 ];
    // Extract the address portion of &arr
    let addr1_usize : usize = get_address(&arr);

    // Create a new capability with the provenance of &arr to the address of arr[1]
    let mut new_cap : *const i64 = new_cap_with_provenance(
        &arr as *const i64,
        addr1_usize + size_of::<i64>());

    // Restrict bounds of new_cap to size_of i64 (8 bytes)
    new_cap = set_bounds(new_cap, size_of::<i64>());

    // Restrict pemissions of new_cap to load only
    new_cap = set_perm(new_cap, CHERI_CAP_PERMISSION_PERMIT_LOAD);
    
    println!("&arr    bounds: {}", get_length(&arr));
    println!("new_cap bounds: {}", get_length(new_cap));
    unsafe {
        // Dereference our restricted capability
        println!("{}", *new_cap);
    }
}
