// Copyright 2015 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Building blocks for parsing DER-encoded ASN.1 structures.
//!
//! This module contains the foundational parts of an ASN.1 DER parser.

use super::input::*;

pub const CONSTRUCTED : u8 = 1 << 5;
pub const CONTEXT_SPECIFIC : u8 = 2 << 6;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Tag {
    Boolean = 0x01,
    Integer = 0x02,
    BitString = 0x03,
    OctetString = 0x04,
    Null = 0x05,
    OID = 0x06,
    Sequence = CONSTRUCTED | 0x10, // 0x30
    UTCTime = 0x17,
    GeneralizedTime = 0x18,

    ContextSpecificConstructed0 = CONTEXT_SPECIFIC | CONSTRUCTED | 0,
    ContextSpecificConstructed1 = CONTEXT_SPECIFIC | CONSTRUCTED | 1,
    ContextSpecificConstructed3 = CONTEXT_SPECIFIC | CONSTRUCTED | 3,
}

pub fn expect_tag_and_get_value<'a>(input: &mut Reader<'a>, tag: Tag)
                                    -> Result<Input<'a>, ()> {
    let (actual_tag, inner) = try!(read_tag_and_get_value(input));
    if (tag as usize) != (actual_tag as usize) {
        return Err(());
    }
    Ok(inner)
}

pub fn read_tag_and_get_value<'a>(input: &mut Reader<'a>)
                                  -> Result<(u8, Input<'a>), ()> {
    let tag = try!(input.read_byte());
    if (tag & 0x1F) == 0x1F {
        return Err(()) // High tag number form is not allowed.
    }

    // If the high order bit of the first byte is set to zero then the length
    // is encoded in the seven remaining bits of that byte. Otherwise, those
    // seven bits represent the number of bytes used to encode the length.
    let length = match try!(input.read_byte()) {
        n if (n & 0x80) == 0 => n as usize,
        0x81 => {
            let second_byte = try!(input.read_byte());
            if second_byte < 128 {
                return Err(()) // Not the canonical encoding.
            }
            second_byte as usize
        },
        0x82 => {
            let second_byte = try!(input.read_byte()) as usize;
            let third_byte = try!(input.read_byte()) as usize;
            let combined = (second_byte << 8) | third_byte;
            if combined < 256 {
                return Err(()); // Not the canonical encoding.
            }
            combined
        },
        _ => {
            return Err(()); // We don't support longer lengths.
        }
    };

    let inner = try!(input.skip_and_get_input(length));
    Ok((tag, inner))
}

// TODO: investigate taking decoder as a reference to reduce generated code
// size.
pub fn nested<'a, F, R, E: Copy>(input: &mut Reader<'a>, tag: Tag, error: E,
                                 decoder: F) -> Result<R, E>
                                 where F : FnOnce(&mut Reader<'a>)
                                 -> Result<R, E> {
    let inner = try!(expect_tag_and_get_value(input, tag).map_err(|_| error));
    read_all(inner, error, decoder)
}

pub fn positive_integer<'a>(input: &mut Reader<'a>) -> Result<Input<'a>, ()> {
    let value = try!(expect_tag_and_get_value(input, Tag::Integer));
    let bytes = value.as_slice_less_safe();

    // Empty encodings are not allowed.
    if bytes.len() == 0 {
        return Err(());
    }

    // Negative values are not allowed.
    if bytes[0] & 0xf0 != 0 {
        return Err(());
    }

    // Over-long encodings are not allowed.
    if bytes.len() > 1 && bytes[0] == 0 && (bytes[1] & 0xf0 == 0) {
        return Err(());
    }

    Ok(value)
}
