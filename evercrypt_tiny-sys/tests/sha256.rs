use evercrypt_tiny_sys::{EverCrypt_Hash_hash, Spec_Hash_Definitions_SHA2_224, Spec_Hash_Definitions_hash_alg, EverCrypt_AEAD_encrypt, EverCrypt_AEAD_state_s_s};

#[test]
fn sha224() -> u32 {
    const ALGO: Spec_Hash_Definitions_hash_alg = Spec_Hash_Definitions_SHA2_224 as _;
    const INPUT: &str = "The quick brown fox jumps over the lazy dog";


    const HASH: &[u8] = &[
        0x73, 0x0e, 0x10, 0x9b, 0xd7, 0xa8, 0xa3, 0x2b, 0x1c, 0xb9, 0xd9, 0xa0, 0x9a, 0xa2, 0x32, 0x5d, 0x24, 0x30,
        0x58, 0x7d, 0xdb, 0xc0, 0xc3, 0x8b, 0xad, 0x91, 0x15, 0x25,
    ];
    
    let mut key = vec![0; 128];

    let mut buf = vec![0; 28];
    let mut inv = vec![0; 48];
    let mut ad = vec![0; 48];
    let mut tag = vec![0; 48];
    let mut input = INPUT.as_bytes().to_vec();
    let mut output = INPUT.as_bytes().to_vec();
    unsafe { EverCrypt_Hash_hash(ALGO, buf.as_mut_ptr(), input.as_mut_ptr(), input.len() as u32) };
    assert_eq!(buf.as_slice(), HASH);
    unsafe{
        let mut aead_state = vec![key.as_mut_ptr() as u64; 1];
        EverCrypt_AEAD_encrypt(
        aead_state.as_mut_ptr() as *mut EverCrypt_AEAD_state_s_s,
        inv.as_mut_ptr(),
        48,
        ad.as_mut_ptr(),
        48,
        input.as_mut_ptr(),
        input.len() as u32,
        output.as_mut_ptr(),
        tag.as_mut_ptr(),
        )
    }
}
