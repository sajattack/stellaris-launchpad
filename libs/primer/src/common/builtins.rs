//! Functions required by the code rustc/LLVM generates.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use rlibc;
use core;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Required by the compiler.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
    loop {}
}

/// Required by the compiler.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> ! {
    loop {}
}

/// Required by modules that haven't been build with panic = "abort"
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

/// Required by the compiler.
#[lang="eh_personality"]
extern "C" fn eh_personality() {}

/// Required by LLVM ... sometimes
#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr4(s: *mut u8, n: usize) -> *mut u8 {
    rlibc::memset(s, 0, n)
}

/// Required by LLVM ... sometimes
#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr(s: *mut u8, n: usize) -> *mut u8 {
    rlibc::memset(s, 0, n)
}

/// Required by LLVM ... sometimes
#[no_mangle]
pub unsafe extern "C" fn __aeabi_memmove(dest: *mut u8, src: *mut u8, n: usize) {
    rlibc::memmove(dest, src, n);
}

/// Entry point of panic from the libcore crate.
///
/// Required by the compiler.
#[lang="panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************