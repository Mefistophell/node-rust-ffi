extern crate libc;

use libc::c_char;
use std::ffi::CString;
use std::ffi::CStr;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Concatenates an input string with an existing string literal
///
/// # Arguments
///
/// * input : a string (actually a raw pointer representing C-like char type)
pub extern "C" fn hello(input: *const c_char) -> *const c_char {
    let input_cstring: &CStr = unsafe {
        // Wraps a raw C-string with a safe C string wrapper
        // Function is unsafe
        CStr::from_ptr(input)
    };
    // Converts a valid UTF-8 CStr into a string slice
    let input_str: &str = input_cstring.to_str().unwrap();
    // String concatenation
    let output_string: String = format!("Hello {} from Rust", input_str);

    // Returns c_char
    CString::new(output_string).unwrap().into_raw()
}

#[repr(C)]
/// Output struct
///
/// # Fields
///
/// * result : [`i64`]
/// * operands : [[`i64`]]
/// * description : `*const`[`c_char`]
pub struct Output {
    result: i64,
    operands: [i64; 2],
    description: *const c_char,
}

impl Output {
    fn multiplication(operation: Operation) -> Output {
        let description: String = format!("{} multiplied by {} is {}", operation.operand_a, operation.operand_b, operation.result);
        Output {
            result: operation.result,
            operands: [operation.operand_a, operation.operand_b],
            description: CString::new(description).unwrap().into_raw(),
        }
    }
}

#[repr(C)]
/// Operation struct
///
/// # Fields
///
/// * operand_a : [`i64`]
/// * operand_b : [`i64`]
/// * result : [`i64`]
pub struct Operation {
    operand_a: i64,
    operand_b: i64,
    result: i64,
}

impl Operation {
    fn multiply(&mut self) {
        self.result = self.operand_a * self.operand_b;
    }
}

/// Performs multiplication of two numbers and return the [`Output`] struct
///
/// # Arguments
///
/// * [`Operation`] the Operation struct
///
#[no_mangle]
pub extern fn multiply(mut operation: Operation) -> Output {
    operation.multiply();
    Output::multiplication(operation)
}
