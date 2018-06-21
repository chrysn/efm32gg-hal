//! Bit-banding access for peripheral registers
//!
//! The addresses for this are derived from the Cortex-M4 generic guide, but the EFM32-Cortex-M3
//! guide uses the same addresses.
//!
//! The implementation is copied over from the tm4c123x-hal crate, pending generalization.
//!
//! (The original code is licensed under the same terms and available on
//! <https://github.com/thejpster/tm4c123x-hal>)

use core::ptr::write_volatile;

/// Sets/Clears a bit at the given address atomically, using the bit-banding
/// feature. We take a const pointer and mutate it, but that's because the
/// svd2rust crate will only give us const pointers.
pub unsafe fn change_bit<T>(address: *const T, bit: u8, value: bool) {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    write_volatile(bit_word, if value { 0x01 } else { 0x00 });
}

/// Address must be >= 0x2000_0000 and <= 0x2007_FFFC. Bit must be < 32.
fn ref_to_bitband(address: u32, bit: u8) -> *mut u32 {
    let prefix = address & 0xF000_0000;
    let byte_offset = address & 0x0FFF_FFFF;
    let bit_word_offset = (byte_offset * 32) + (bit as u32 * 4);
    let bit_word_addr = bit_word_offset + prefix + 0x0200_0000;
    bit_word_addr as *mut u32
}
