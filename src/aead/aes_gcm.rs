// Copyright 2015-2016 Brian Smith.
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

#![allow(unsafe_code)]

use super::super::{aead, bssl, c, polyfill};

const AES_128_KEY_LEN: usize = 128 / 8;
const AES_256_KEY_LEN: usize = 32; // 256 / 8

/// AES-128 in GCM mode with 128-bit tags and 96 bit nonces.
///
/// C analog: `EVP_aead_aes_128_gcm`
///
/// Go analog: [`crypto.aes`](https://golang.org/pkg/crypto/aes/)
pub static AES_128_GCM: aead::Algorithm = aead::Algorithm {
    key_len: AES_128_KEY_LEN,
    init: aes_gcm_init,
    seal: aes_gcm_seal,
    open: aes_gcm_open,
};

/// AES-256 in GCM mode with 128-bit tags and 96 bit nonces.
///
/// C analog: `EVP_aead_aes_256_gcm`
///
/// Go analog: [`crypto.aes`](https://golang.org/pkg/crypto/aes/)
pub static AES_256_GCM: aead::Algorithm = aead::Algorithm {
    key_len: AES_256_KEY_LEN,
    init: aes_gcm_init,
    seal: aes_gcm_seal,
    open: aes_gcm_open,
};

fn aes_gcm_init(ctx_buf: &mut [u8], key: &[u8]) -> Result<(), ()> {
    bssl::map_result(unsafe {
        evp_aead_aes_gcm_init(ctx_buf.as_mut_ptr(),
                              ctx_buf.len(), key.as_ptr(), key.len())
    })
}

fn aes_gcm_seal(ctx: &[u64; aead::KEY_CTX_BUF_ELEMS],
                nonce: &[u8; aead::NONCE_LEN], in_out: &mut [u8],
                tag: &mut [u8; aead::TAG_LEN], ad: &[u8]) -> Result<(), ()> {
    let ctx = polyfill::slice::u64_as_u8(ctx);
    bssl::map_result(unsafe {
        evp_aead_aes_gcm_seal(ctx.as_ptr(), in_out.as_mut_ptr(), in_out.len(),
                              tag.as_mut_ptr(), nonce.as_ptr(), ad.as_ptr(),
                              ad.len())
    })
}

fn aes_gcm_open(ctx: &[u64; aead::KEY_CTX_BUF_ELEMS],
                nonce: &[u8; aead::NONCE_LEN], in_out: &mut [u8],
                in_prefix_len: usize, tag_out: &mut [u8; aead::TAG_LEN],
                ad: &[u8]) -> Result<(), ()> {
    let ctx = polyfill::slice::u64_as_u8(ctx);
    bssl::map_result(unsafe {
        evp_aead_aes_gcm_open(ctx.as_ptr(), in_out.as_mut_ptr(),
                              in_out.len() - in_prefix_len,
                              tag_out.as_mut_ptr(), nonce.as_ptr(),
                              in_out[in_prefix_len..].as_ptr(), ad.as_ptr(),
                              ad.len())
    })
}

extern {
    fn evp_aead_aes_gcm_init(ctx_buf: *mut u8, ctx_buf_len: c::size_t,
                             key: *const u8, key_len: c::size_t) -> c::int;

    fn evp_aead_aes_gcm_seal(ctx_buf: *const u8, in_out: *mut u8,
                             in_out_len: c::size_t,
                             tag_out: *mut u8/*[TAG_LEN]*/,
                             nonce: *const u8/*[TAG_LEN]*/, ad: *const u8,
                             ad_len: c::size_t) -> c::int;

    fn evp_aead_aes_gcm_open(ctx_buf: *const u8, out: *mut u8,
                             in_out_len: c::size_t,
                             tag_out: *mut u8/*[TAG_LEN]*/,
                             nonce: *const u8/*[NONCE_LEN]*/,
                             in_: *const u8, ad: *const u8, ad_len: c::size_t)
                             -> c::int;
}
