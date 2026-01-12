#![allow(dead_code, unused_imports, unused_variables, unused_parens)]
#![deny(clippy::shadow_reuse, clippy::shadow_same, clippy::shadow_unrelated)]

use std::ops::Deref;

fn main() {
  //REM: BEGIN; Scalar data type (primitive data type)
  {
    let boolean: bool = false | true; //REM: 1 byte

    let signed_byte: i8 = -128 | 127; //REM: signed 1 byte
    let unsigned_byte: u8 = 0 | 255; //REM: unsigned 1 byte

    let signed_short: i16 = -32_758 | 32_757; //REM: signed 2 bytes
    let unsigned_short: u16 = 0 | 65_535; //REM: unsigned 2 bytes

    //REM: signed 4 bytes
    let signed_char_min: char = '\0'; //REM: 0 base 10, null-terminated
    let signed_char_max: char = '\u{10FFFF}'; //REM: ~1M base 10, ???

    let signed_int: i32 = -2_147_483_648 | 2_147_483_647; //REM: signed 4 bytes
    let unsigned_int: u32 = 0 | 4_294_967_295; //REM: unsigned 4 bytes

    let signed_long: i64 =
      -9_223_372_036_854_775_808 | 9_223_372_036_854_775_807; //REM: signed 8 bytes
    let unsigned_long: u64 = 0 | 18_446_744_073_709_551_615; //REM: unsigned 8 bytes

    let signed_16_bytes: i128 = 
      -170_141_183_460_469_231_731_687_303_715_884_105_728 
      | 170_141_183_460_469_231_731_687_303_715_884_105_727; //REM: signed 16 bytes
    let unsigned_16_bytes: u128 = 
      0 | 340_282_366_920_938_463_463_374_607_431_768_211_455; //REM: unsigned 16 bytes

    let signed_float: f32 = ((-3.4028235e38 as f32).to_bits()
      | (3.4028235e38 as f32).to_bits()) as f32; //REM: signed 4 bytes
    let signed_double: f64 = ((-1.7976931348623157e308 as f64).to_bits()
      | (1.7976931348623157e308 as f64).to_bits())
      as f64; //REM: signed 8 bytes

    //REM: Usually signed 32 bits, 64 bits, or any architecture dependent data types alignment
    let signed_size: isize = std::isize::MIN | std::isize::MAX;
    //REM: Usually unsigned 32 bits, 64 bits, or any architecture dependent data types alignment
    let unsigned_size: usize = std::usize::MIN | std::usize::MAX;
  } //REM: END; Scalar data type

  //REM: BEGIN; Compound data type
  {
    let tuple: (i8,f32,&str,String) 
      = (1,2.0,"Three","FOUR".to_string());
    let (i, ii, iii, iv) = tuple;

    let array_i8: [i8;4] = [1,2,3,4];
    let i8: i8 = array_i8[3]; //REM: 4

    let array_str: [&str;4] = ["1","2","3","4"];
    let array_str_i: [&str;_] = ["1","2","3","4"];
    let array_str_ii: [&str;_] = ["0";4]; //REM: ["0","0","0","0"]
    let array_str_iii: &[&str] = &array_str_i[2..=3]; //REM: ["3","4"]

  } //REM: END; Compound data type

  //REM: BEGIN; Constant and Static
  {
    const CONSTANT_X: i8 = 10;

    static STATIC_Y: &str = "YYY";
    
    static mut STATIC_Z: f32 = 26.0;
    unsafe {
      STATIC_Z = 0.0;
    }
  } //REM: END; Constant

  //REM: BEGIN; Basic String handling
  {
    let mut string_y: String = "string_y".to_string();
    let string_y_i: String = "string_y_i".to_owned();
    let string_y_ii: String = String::from("string_y_ii");

    let str_x: &str = "str_x";
    let str_x_i: &str = string_y.as_str();
    let str_x_ii: &mut str = string_y.as_mut_str();
    let str_x_iii: &str = &str_x[..]; //REM: 'str_x'
    let str_x_iv: &str = &str_x[..=2]; //REM: 'str'
    let str_x_v: &str = &str_x[3..]; //REM: '_x'

    unsafe {
      let str_x_vi: &mut [u8] = str_x_ii.as_bytes_mut();
      str_x_vi[0] = b'S';
    
      let str_x_vii: *mut u8 = string_y.as_mut_ptr();
      str_x_vii.add(1).write(b'T');
    }
  } //REM: END; Basic String handling
}
