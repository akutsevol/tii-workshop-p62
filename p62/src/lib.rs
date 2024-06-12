use std::arch::x86_64::*;
use std::mem;

#[allow(unused_imports)]
use rand::Rng;

const NUM_ROUNDS: usize = 20;

const fn my_mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[repr(C)]
pub struct Aes128KeySchedule {
    rounds: [__m128i; NUM_ROUNDS],
}

#[no_mangle]
pub extern "C" fn aes128_load_key(key: *const u8) -> *mut Aes128KeySchedule {
    let mut key_schedule: Aes128KeySchedule = unsafe { mem::zeroed() };
    
    unsafe {
        key_schedule.rounds[0] = _mm_loadu_si128(key as *const __m128i);
        key_schedule.rounds[1] = aes_128_key_expansion(key_schedule.rounds[0], _mm_aeskeygenassist_si128(key_schedule.rounds[0], 0x01));
        key_schedule.rounds[2] = aes_128_key_expansion(key_schedule.rounds[1], _mm_aeskeygenassist_si128(key_schedule.rounds[1], 0x02));
        key_schedule.rounds[3] = aes_128_key_expansion(key_schedule.rounds[2], _mm_aeskeygenassist_si128(key_schedule.rounds[2], 0x04));
        key_schedule.rounds[4] = aes_128_key_expansion(key_schedule.rounds[3], _mm_aeskeygenassist_si128(key_schedule.rounds[3], 0x08));
        key_schedule.rounds[5] = aes_128_key_expansion(key_schedule.rounds[4], _mm_aeskeygenassist_si128(key_schedule.rounds[4], 0x10));
        key_schedule.rounds[6] = aes_128_key_expansion(key_schedule.rounds[5], _mm_aeskeygenassist_si128(key_schedule.rounds[5], 0x20));
        key_schedule.rounds[7] = aes_128_key_expansion(key_schedule.rounds[6], _mm_aeskeygenassist_si128(key_schedule.rounds[6], 0x40));
        key_schedule.rounds[8] = aes_128_key_expansion(key_schedule.rounds[7], _mm_aeskeygenassist_si128(key_schedule.rounds[7], 0x80));
        key_schedule.rounds[9] = aes_128_key_expansion(key_schedule.rounds[8], _mm_aeskeygenassist_si128(key_schedule.rounds[8], 0x1B));
        key_schedule.rounds[10] = aes_128_key_expansion(key_schedule.rounds[9], _mm_aeskeygenassist_si128(key_schedule.rounds[9], 0x36));

        key_schedule.rounds[11] = _mm_aesimc_si128(key_schedule.rounds[9]);
        key_schedule.rounds[12] = _mm_aesimc_si128(key_schedule.rounds[8]);
        key_schedule.rounds[13] = _mm_aesimc_si128(key_schedule.rounds[7]);
        key_schedule.rounds[14] = _mm_aesimc_si128(key_schedule.rounds[6]);
        key_schedule.rounds[15] = _mm_aesimc_si128(key_schedule.rounds[5]);
        key_schedule.rounds[16] = _mm_aesimc_si128(key_schedule.rounds[4]);
        key_schedule.rounds[17] = _mm_aesimc_si128(key_schedule.rounds[3]);
        key_schedule.rounds[18] = _mm_aesimc_si128(key_schedule.rounds[2]);
        key_schedule.rounds[19] = _mm_aesimc_si128(key_schedule.rounds[1]);
    }
    
    let boxed_schedule = Box::new(key_schedule);
    Box::into_raw(boxed_schedule)
}

unsafe fn aes_128_key_expansion(key: __m128i, keygened: __m128i) -> __m128i {
    let keygened = _mm_shuffle_epi32(keygened, my_mm_shuffle(3, 3, 3, 3));
    let mut key = key;
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    _mm_xor_si128(key, keygened)
}

// #[no_mangle]
// pub extern "C" fn aes128_load_key(key: *const u8) -> [__m128i; NUM_ROUNDS] {
//     let key = unsafe { std::slice::from_raw_parts(key, 16) };
//     let mut key_schedule = [unsafe { _mm_setzero_si128() }; NUM_ROUNDS];
//     unsafe {
//         key_schedule[0] = _mm_loadu_si128(key.as_ptr() as *const __m128i);
//         key_schedule[1] = aes_128_key_expansion(key_schedule[0], _mm_aeskeygenassist_si128(key_schedule[0], 0x01));
//         key_schedule[2] = aes_128_key_expansion(key_schedule[1], _mm_aeskeygenassist_si128(key_schedule[1], 0x02));
//         key_schedule[3] = aes_128_key_expansion(key_schedule[2], _mm_aeskeygenassist_si128(key_schedule[2], 0x04));
//         key_schedule[4] = aes_128_key_expansion(key_schedule[3], _mm_aeskeygenassist_si128(key_schedule[3], 0x08));
//         key_schedule[5] = aes_128_key_expansion(key_schedule[4], _mm_aeskeygenassist_si128(key_schedule[4], 0x10));
//         key_schedule[6] = aes_128_key_expansion(key_schedule[5], _mm_aeskeygenassist_si128(key_schedule[5], 0x20));
//         key_schedule[7] = aes_128_key_expansion(key_schedule[6], _mm_aeskeygenassist_si128(key_schedule[6], 0x40));
//         key_schedule[8] = aes_128_key_expansion(key_schedule[7], _mm_aeskeygenassist_si128(key_schedule[7], 0x80));
//         key_schedule[9] = aes_128_key_expansion(key_schedule[8], _mm_aeskeygenassist_si128(key_schedule[8], 0x1B));
//         key_schedule[10] = aes_128_key_expansion(key_schedule[9], _mm_aeskeygenassist_si128(key_schedule[9], 0x36));

//         key_schedule[11] = _mm_aesimc_si128(key_schedule[9]);
//         key_schedule[12] = _mm_aesimc_si128(key_schedule[8]);
//         key_schedule[13] = _mm_aesimc_si128(key_schedule[7]);
//         key_schedule[14] = _mm_aesimc_si128(key_schedule[6]);
//         key_schedule[15] = _mm_aesimc_si128(key_schedule[5]);
//         key_schedule[16] = _mm_aesimc_si128(key_schedule[4]);
//         key_schedule[17] = _mm_aesimc_si128(key_schedule[3]);
//         key_schedule[18] = _mm_aesimc_si128(key_schedule[2]);
//         key_schedule[19] = _mm_aesimc_si128(key_schedule[1]);
//     }
//     key_schedule
// }

#[no_mangle]
pub extern "C" fn aes128_encode(plain_text: *const u8, cipher_text: *mut u8, key_schedule: *const __m128i) {
    let plain_text = unsafe { std::slice::from_raw_parts(plain_text, 16) };
    let cipher_text = unsafe { std::slice::from_raw_parts_mut(cipher_text, 16) };
    let key_schedule = unsafe { std::slice::from_raw_parts(key_schedule, NUM_ROUNDS) };

    unsafe {
        let mut m = _mm_loadu_si128(plain_text.as_ptr() as *const __m128i);
        m = _mm_xor_si128(m, key_schedule[0]);
        m = _mm_aesenc_si128(m, key_schedule[1]);
        m = _mm_aesenc_si128(m, key_schedule[2]);
        m = _mm_aesenc_si128(m, key_schedule[3]);
        m = _mm_aesenc_si128(m, key_schedule[4]);
        m = _mm_aesenc_si128(m, key_schedule[5]);
        m = _mm_aesenc_si128(m, key_schedule[6]);
        m = _mm_aesenc_si128(m, key_schedule[7]);
        m = _mm_aesenc_si128(m, key_schedule[8]);
        m = _mm_aesenc_si128(m, key_schedule[9]);
        m = _mm_aesenclast_si128(m, key_schedule[10]);
        _mm_storeu_si128(cipher_text.as_mut_ptr() as *mut __m128i, m);
    }
}

#[no_mangle]
pub extern "C" fn aes128_decode(cipher_text: *const u8, plain_text: *mut u8, key_schedule: *const __m128i) {
    let cipher_text = unsafe { std::slice::from_raw_parts(cipher_text, 16) };
    let plain_text = unsafe { std::slice::from_raw_parts_mut(plain_text, 16) };
    let key_schedule = unsafe { std::slice::from_raw_parts(key_schedule, NUM_ROUNDS) };

    unsafe {
        let mut m = _mm_loadu_si128(cipher_text.as_ptr() as *const __m128i);
        m = _mm_xor_si128(m, key_schedule[10 + 0]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 1]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 2]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 3]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 4]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 5]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 6]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 7]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 8]);
        m = _mm_aesdec_si128(m, key_schedule[10 + 9]);
        m = _mm_aesdeclast_si128(m, key_schedule[0]);
        _mm_storeu_si128(plain_text.as_mut_ptr() as *mut __m128i, m);
    }
}

#[no_mangle]
pub extern "C" fn aes128_free_key_schedule(schedule: *mut Aes128KeySchedule) {
    if !schedule.is_null() {
        unsafe {
            let _ = Box::from_raw(schedule);
        }
    }
}