#![allow(dead_code, unused_imports, unused_variables, unused_parens)]
#![deny(clippy::shadow_reuse, clippy::shadow_same, clippy::shadow_unrelated)]

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
  {} //REM: END; Compound data type

  //REM: BEGIN; Constant
  {} //REM: END; Constant

  //REM: BEGIN; Basic String handling
  {} //REM: END; Basic String handling
}
