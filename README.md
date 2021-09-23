[![crates.io](https://img.shields.io/crates/v/null_fn.svg)](https://crates.io/crates/null_fn)

# ✨ `null_fn`

A proc attribute macro that allows for creating null function pointers in `static`s.

## Example

```rust
static mut UTIL_PlayerByUserId: unsafe extern "C" fn(userid: i32) -> *mut c_void = unsafe { std::mem::transmute::<*const (), _>(std::ptr::null()) }; // error[E0080]: it is undefined behavior to use this value

#[null_fn]
static mut UTIL_PlayerByUserId: unsafe extern "C" fn(userid: i32) -> *mut c_void = std::ptr::null(); // works!

fn main() {
	unsafe {
		UTIL_PlayerByUserId(20); // This would panic, as the pointer is NULL.

		let is_null = UTIL_PlayerByUserId.is_null(); // It's just a normal pointer, so we can call pointer-related functions on it.

		UTIL_PlayerByUserId = /* magically find the pointer to the function */;
		let player = UTIL_PlayerByUserId(20); // Now that we set the function pointer, we can call the function without panicking.
	}
}
```
