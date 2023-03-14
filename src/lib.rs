#![feature(link_llvm_intrinsics)]
#![feature(abi_unadjusted)]

// reconsider?
pub type Address = usize;
pub type Capability = *const i8;
pub enum Perms { ReadWriteExecute, ReadWrite, ReadExecute, WriteExecute, Read, Write, Execute }
pub enum Flags {  }

// extern "C" {
//     fn cheri_getlen(cap: capability) -> size_t;
//     fn cheri_getbase(cap: capability) -> Address;
//     fn cheri_getoffset(cap: capability) -> size_t;
//     fn cheri_getaddress(cap: capability) -> Address;
//     fn cheri_set_address(cap: capability, addr: Address) -> capability;
//     fn cheri_getflags(cap: capability) -> usize;
//     fn cheri_getperms(cap: capability) -> usize;
//     fn cheri_getsealed(cap: capability) -> bool;
//     fn cheri_gettag(cap: capability) -> bool;
//     fn cheri_gettype(cap: capability) -> i64;
// }



// pub unsafe fn get_tag<T>(cap: *mut T) -> bool {
//     return __builtin_cheri_tag_get(cap as Capability);
// }

pub fn get_address<T>(cap: *mut T) -> Address {
    extern "unadjusted" {
        #[link_name = "llvm.cheri.cap.address.get.i64"]
        fn __builtin_cheri_address_get(cap: Capability) -> Address;    
    }

    return unsafe { __builtin_cheri_address_get(cap as Capability) }
}

pub fn new_cap_with_provenance<T>(cap: *mut T, addr: Address) -> *mut T {
    extern "unadjusted" {
        #[link_name = "llvm.cheri.cap.address.set.i64"]
        fn __builtin_cheri_address_set(cap: Capability, addr: Address) -> Capability;
    }
    
    return unsafe { __builtin_cheri_address_set(cap as Capability, addr) as *mut T };
}
