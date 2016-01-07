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

#![no_std]

#[cfg(test)]
#[macro_use(format, print, println, vec)]
extern crate std;

#[cfg(test)]
extern crate rustc_serialize;

pub mod aead;
pub mod agreement;
mod c;
pub mod constant_time;

#[doc(hidden)]
pub mod der;

pub mod digest;
mod ecc;
mod ffi;
pub mod hkdf;
pub mod hmac;
pub mod input;
pub mod pbkdf2;
mod polyfill;
pub mod rand;
pub mod signature;

#[cfg(test)]
mod exe_tests;

#[cfg(test)]
mod file_test;
