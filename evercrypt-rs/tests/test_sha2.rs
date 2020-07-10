use evercrypt::digest::{Digest, Mode};

#[test]
fn test_sha() {
    let data = b"evercrypt-rust bindings";
    let d = Digest::hash(Mode::Sha256, data);
    let expected_digest_256 = [
        0xa5, 0x35, 0xf2, 0x6a, 0xff, 0xbc, 0x1f, 0x08, 0x73, 0xdb, 0x15, 0x15, 0x9d, 0xce, 0xbf,
        0x25, 0x99, 0x64, 0xbe, 0x42, 0xde, 0xa8, 0x4d, 0x29, 0x00, 0x38, 0x4b, 0xee, 0x15, 0x09,
        0xe4, 0x00,
    ];
    let expected_digest_512 = [
        0x36, 0x97, 0x36, 0x7c, 0xc9, 0x1e, 0xda, 0xa7, 0x6d, 0xb8, 0x03, 0x39, 0x61, 0x5f, 0xc2,
        0x12, 0xe1, 0x5e, 0x64, 0x3e, 0x31, 0x30, 0xf7, 0x1f, 0x28, 0xd0, 0x3f, 0x34, 0x3d, 0xf4,
        0x88, 0x0a, 0xd3, 0x6c, 0x63, 0xe5, 0x35, 0x1f, 0x56, 0xe0, 0xf7, 0xe0, 0x4c, 0x24, 0x96,
        0xc0, 0xb3, 0x6b, 0xcf, 0x7c, 0x5d, 0xcb, 0xf3, 0x5e, 0x38, 0xe9, 0xbb, 0x44, 0xf8, 0xa0,
        0xc2, 0x83, 0x42, 0x4e,
    ];
    assert_eq!(d, expected_digest_256);
    assert_eq!(Digest::hash(Mode::Sha512, data)[..], expected_digest_512[..]);

    let mut digest = Digest::new(Mode::Sha256);
    assert!(digest.update(data).is_ok());
    match digest.finish() {
        Ok(d) => assert_eq!(d, expected_digest_256),
        Err(r) => panic!("Got error in finish {:?}", r),
    }
    assert!(digest.finish().is_err());
    assert!(digest.update(&[]).is_err());

    let mut digest = Digest::new(Mode::Sha512);
    assert!(digest.update(data).is_ok());
    match digest.finish() {
        Ok(d) => assert_eq!(d[..], expected_digest_512[..]),
        Err(r) => panic!("Got error in finish {:?}", r),
    }
    assert!(digest.finish().is_err());
    assert!(digest.update(&[]).is_err());
}

