// MIT License

// Copyright (c) 2019-2020 brycx

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate orion;

use orion::hazardous::stream::chacha20::{SecretKey, CHACHA_KEYSIZE};

#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(CHACHA_KEYSIZE as i32);
    let zero_sk = SecretKey::from_slice(&[0u8; CHACHA_KEYSIZE]).unwrap();
    let gen_sk = SecretKey::from_slice(input).unwrap();

    sidefuzz::black_box(secret_key_pt_eq(&zero_sk, &gen_sk));
}

fn secret_key_pt_eq(zero_sk: &SecretKey, gen_sk: &SecretKey) -> bool {
    zero_sk == gen_sk
}
