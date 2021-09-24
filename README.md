[![crates.io](https://img.shields.io/crates/v/null_fn.svg)](https://crates.io/crates/null_fn)

# ✨ `null_fn`

A proc attribute macro that allows for creating null function pointers in `static`s.

This crate is **unsafe and easy to cause UB with**, `Option<fn()>` is [FFI safe](https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization) and may be a more appropriate alternative if you value type safety.

## Example

```rust
static mut UTIL_PlayerByUserId: unsafe extern "C" fn(userid: i32) -> *mut c_void = unsafe { std::mem::transmute::<*const (), _>(std::ptr::null()) }; // error[E0080]: it is undefined behavior to use this value

#[null_fn]
static mut UTIL_PlayerByUserId: unsafe extern "C" fn(userid: i32) -> *mut c_void = std::ptr::null(); // works!

fn main() {
    unsafe {
        UTIL_PlayerByUserId(20); // This would panic, as we have not initialized the function yet. By default the function is set to a small stub function that panics when called.

        UTIL_PlayerByUserId = /* magically find the pointer to the function; sigscan? */;
		// Setting the function's pointer to a null pointer is UB in Rust.
		// https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization

        let player = UTIL_PlayerByUserId(20); // Now that we set the function pointer, we can call the function without panicking, assuming we found the pointer correctly.

		/* do something with our player! */
    }
}
```
