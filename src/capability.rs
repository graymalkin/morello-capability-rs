// reconsider?
pub type Address = usize;
pub type Capability = *const i8;

pub const CHERI_CAP_PERMISSION_GLOBAL : i64 = 1 << 0;
pub const ARM_CAP_PERMISSION_EXECUTIVE : i64 = 1 << 1;
pub const ARM_CAP_PERMISSION_MUTABLE_LOAD : i64 = 1 << 6;
pub const ARM_CAP_PERMISSION_COMPARTMENT_ID : i64 = 1 << 7;
pub const ARM_CAP_PERMISSION_BRANCH_SEALED_PAIR : i64 = 1 << 8;
pub const CHERI_CAP_PERMISSION_ACCESS_SYSTEM_REGISTERS : i64 = 1 << 9;
pub const CHERI_CAP_PERMISSION_PERMIT_UNSEAL : i64 = 1 << 10;
pub const CHERI_CAP_PERMISSION_PERMIT_SEAL : i64 = 1 << 11;
pub const CHERI_CAP_PERMISSION_PERMIT_STORE_LOCAL : i64 = 1 << 12;
pub const CHERI_CAP_PERMISSION_PERMIT_STORE_CAPABILITY : i64 = 1 << 13;
pub const CHERI_CAP_PERMISSION_PERMIT_LOAD_CAPABILITY : i64 = 1 << 14;
pub const CHERI_CAP_PERMISSION_PERMIT_EXECUTE : i64 = 1 << 15;
pub const CHERI_CAP_PERMISSION_PERMIT_STORE : i64 = 1 << 16;
pub const CHERI_CAP_PERMISSION_PERMIT_LOAD : i64 = 1 << 17;


/// Get the address of a capability
/// 
/// This uses the Morello intrinsic and returns just the `Address` portion of a capability.
pub fn get_address<T>(cap: *const T) -> Address {
    extern {
        #[link_name = "llvm.cheri.cap.address.get.i64"]
        fn __builtin_cheri_address_get(cap: Capability) -> Address;
    }

    return unsafe { __builtin_cheri_address_get(cap as Capability) }
}

/// Get the base address of a capability
pub fn get_base<T>(cap: *const T) -> Address {
    extern {
        #[link_name = "llvm.cheri.cap.base.get.i64"]
        fn __builtin_cheri_base_get(cap: Capability) -> Address;
    }

    return unsafe { __builtin_cheri_base_get(cap as Capability) }
}

/// Get the bounds of a capability
pub fn get_length<T>(cap: *const T) -> usize {
    extern {
        #[link_name = "llvm.cheri.cap.length.get.i64"]
        fn __builtin_cheri_length_get(cap: Capability) -> Address;
    }

    return unsafe { __builtin_cheri_length_get(cap as Capability) }
}

/// Set the bounds on a capability
/// 
/// # Example
/// This creates a bew capability which points to the 2nd element of an array,
/// with bounds restricted to just that element.
/// ```rs
/// let mut arr : [i64; 4] = [ 0, 1, 2, 3 ];
/// let addr1_usize : usize = get_address(&mut arr);
/// let mut new_cap : *mut i64 = new_cap_with_provenance(&mut arr as *mut i64, addr1_usize + mem::size_of::<i64>());
/// new_cap = set_bounds(new_cap, mem::size_of::<i64>());
/// ```
pub fn set_bounds<T>(cap: *const T, len: usize) -> *const T {
    extern {
        #[link_name = "llvm.cheri.cap.bounds.set.i64"]
        fn __builtin_cheri_bounds_set(cap: Capability, len: usize) -> Capability;
    }

    return unsafe { __builtin_cheri_bounds_set(cap as Capability, len) as *const T }
}

/// Get the permissions of a capability.
pub fn get_perms<T>(cap: *const T) -> i64 {
    extern {
        #[link_name = "llvm.cheri.cap.perms.get.i64"]
        fn __builtin_cheri_perms_get(cap: Capability) -> i64;
    }

    return unsafe { __builtin_cheri_perms_get(cap as Capability) }
}

/// Set the permissions of a capability
pub fn set_perm<T>(cap: *const T, perms: i64) -> *const T {
    extern {
        #[link_name = "llvm.cheri.cap.perms.and.i64"]
        fn __builtin_cheri_perms_and(cap: Capability, perms: i64) -> Capability;
    }
    return unsafe { __builtin_cheri_perms_and(cap as Capability, perms) as *const T }
}

/// Clear the tag (validity bit) on a capability
pub fn clear_tag<T>(cap: *const T) -> *const T {
    extern {
        #[link_name = "llvm.cheri.cap.tag.clear"]
        fn __builtin_cheri_tag_clear(cap: Capability) -> Capability;
    }
    return unsafe { __builtin_cheri_tag_clear(cap as Capability) as *const T }

}

pub fn caps_eq_exact<T>(a: *const T, b: *const T) -> bool {
    extern {
        #[link_name = "llvm.cheri.cap.equal.exact"]
        fn __builtin_cheri_eq_exact(a: Capability, b: Capability) -> bool;
    }
    return unsafe { __builtin_cheri_eq_exact(a as Capability, b as Capability) }
}

/// Create a new capability with a specified address using the provenance of another capability.
/// 
/// The bounds of the capability used for provenance must include the address being used.
pub fn new_cap_with_provenance<T>(cap: *const T, addr: Address) -> *const T {
    extern {
        #[link_name = "llvm.cheri.cap.address.set.i64"]
        fn __builtin_cheri_address_set(cap: Capability, addr: Address) -> Capability;
    }
    
    return unsafe { __builtin_cheri_address_set(cap as Capability, addr) as *const T };
}
