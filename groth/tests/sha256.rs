use miracl_core_bls12381::hash256::HASH256;

const EXPECTED_DIGEST: [u8; 32] = [
    0x3a, 0x6e, 0xb0, 0x79, 0x0f, 0x39, 0xac, 0x87, 0xc9, 0x4f, 0x38, 0x56, 0xb2, 0xdd, 0x2c, 0x5d,
    0x11, 0x0e, 0x68, 0x11, 0x60, 0x22, 0x61, 0xa9, 0xa9, 0x23, 0xd3, 0xbb, 0x23, 0xad, 0xc8, 0xb7,
];

#[test]
fn should_return_correct_output_with_single_call_to_write() {
    let mut state = HASH256::new();
    state.process_array(b"data");
    let digest = state.hash();
    assert_eq!(digest, EXPECTED_DIGEST);
}

#[test]
fn should_return_correct_output_with_multiple_calls_to_write() {
    let mut state = HASH256::new();
    state.process_array(b"da");
    state.process_array(b"ta");
    let digest = state.hash();
    assert_eq!(digest, EXPECTED_DIGEST);
}

#[test]
fn should_produce_hash_with_256_bit() {
    let mut state = HASH256::new();
    state.process_array(b"data");
    let digest = state.hash();
    assert_eq!(digest.len(), 256 / 8);
}

#[test]
fn should_produce_hash_with_256_bit_for_data_longer_than_256_bit() {
    let text_with_445_bytes: &[u8; 445] = b"Lorem ipsum dolor sit amet, consectetur \
        adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut \
        enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
        commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum \
        dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
        culpa qui officia deserunt mollit anim id est laborum.";

    let mut state = HASH256::new();
    state.process_array(text_with_445_bytes);
    let digest = state.hash();

    assert_eq!(digest.len(), 256 / 8);
}

#[derive(Debug)]
struct TestContext {
    bytes: Vec<u8>,
}

impl TestContext {
    pub fn new(bytes: &[u8]) -> Self {
        TestContext {
            bytes: bytes.to_vec(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[test]
fn test_sha256_with_nonempty_context_and_nonempty_input() {
    let context = TestContext::new(&[0x11, 0x22, 0x33, 0x44]);
    let data = b"data";

    let mut state = HASH256::new();
    state.process_array(context.as_bytes());
    state.process_array(data);
    let digest = state.hash();

    // macOS: $ echo -n '\x11\x22\x33\x44data' | shasum -a 256
    assert_eq!(
        digest,
        [
            0xec, 0x19, 0xef, 0x13, 0xfe, 0x56, 0x16, 0x62, 0xe0, 0xa8, 0xe2, 0x48, 0x98, 0xd2,
            0xea, 0xf1, 0x10, 0x82, 0x67, 0x62, 0x4e, 0xf7, 0xf6, 0xbb, 0x68, 0x9, 0x1e, 0xac,
            0x1a, 0xd8, 0xf4, 0x5d,
        ]
    );
}

#[test]
fn test_sha256_with_empty_context_and_emtpy_data() {
    let context = TestContext::new(&[]);
    let data = b"";

    let mut state = HASH256::new();
    state.process_array(context.as_bytes());
    state.process_array(data);
    let digest = state.hash();

    // macOS: $ echo -n '' | shasum -a 256
    assert_eq!(
        digest,
        [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ]
    );
}

#[test]
fn test_sha256_with_nonempty_context_and_emtpy_input() {
    let context = TestContext::new(&[0x11, 0x22, 0x33, 0x44]);
    let data = b"";

    let mut state = HASH256::new();
    state.process_array(context.as_bytes());
    state.process_array(data);
    let digest = state.hash();

    // macOS: $ echo -n '\x11\x22\x33\x44' | shasum -a 256
    assert_eq!(
        digest,
        [
            0x1a, 0x83, 0x5e, 0xd8, 0x73, 0x4f, 0x86, 0x35, 0x5c, 0xa5, 0xb8, 0x35, 0xd8, 0x24,
            0xd4, 0x86, 0x99, 0x3a, 0xab, 0xf1, 0x91, 0x3c, 0xd3, 0xa0, 0x11, 0xb7, 0x44, 0x6c,
            0x5, 0x14, 0xb7, 0xc9,
        ]
    );
}

#[test]
fn test_sha256_with_empty_context_and_nonempty_input() {
    let context = TestContext::new(&[]);
    let data = b"data";

    let mut state = HASH256::new();
    state.process_array(context.as_bytes());
    state.process_array(data);
    let digest = state.hash();

    // macOS: $  echo -n 'data' | shasum -a 256
    assert_eq!(
        digest,
        [
            0x3a, 0x6e, 0xb0, 0x79, 0xf, 0x39, 0xac, 0x87, 0xc9, 0x4f, 0x38, 0x56, 0xb2, 0xdd,
            0x2c, 0x5d, 0x11, 0xe, 0x68, 0x11, 0x60, 0x22, 0x61, 0xa9, 0xa9, 0x23, 0xd3, 0xbb,
            0x23, 0xad, 0xc8, 0xb7,
        ]
    );
}
