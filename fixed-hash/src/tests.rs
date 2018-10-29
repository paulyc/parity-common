construct_hash!{ struct H32(4); }
construct_hash!{ struct H64(8); }
construct_hash!{ struct H128(16); }
construct_hash!{ struct H160(20); }
construct_hash!{ struct H256(32); }

impl_hash_conversions!(H256, H160);

mod repeat_byte {
    use super::*;

    #[test]
    fn patterns() {
        assert_eq!(H32::repeat_byte(0xFF), H32::from([0xFF; 4]));
        assert_eq!(H32::repeat_byte(0xAA), H32::from([0xAA; 4]));
    }

    #[test]
    fn zero() {
        assert_eq!(H32::repeat_byte(0x0), H32::zero());
        assert_eq!(H32::repeat_byte(0x0), H32::from([0x0; 4]));
    }
}

#[test]
fn len_bytes() {
    assert_eq!(H32::len_bytes(), 4);
    assert_eq!(H64::len_bytes(), 8);
    assert_eq!(H128::len_bytes(), 16);
    assert_eq!(H160::len_bytes(), 20);
    assert_eq!(H256::len_bytes(), 32);
}

#[test]
fn as_bytes() {
    assert_eq!(H32::from([0x55; 4]).as_bytes(), &[0x55; 4]);
    assert_eq!(H32::from([0x42; 4]).as_bytes_mut(), &mut [0x42; 4]);
}

mod assign_from_slice {
    use super::*;

    #[test]
    fn zeros_to_ones() {
        assert_eq!(
            H32::from([0xFF; 4]),
            {
                let mut cmp = H32::zero();
                cmp.assign_from_slice(&[0xFF; 4]);
                cmp
            }
        );
    }

    #[test]
    #[should_panic]
    fn fail_too_few_elems() {
        let mut dummy = H32::zero();
        dummy.assign_from_slice(&[0x42; 3]);
    }

    #[test]
    #[should_panic]
    fn fail_too_many_elems() {
        let mut dummy = H32::zero();
        dummy.assign_from_slice(&[0x42; 5]);
    }
}

mod from_slice {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(H32::from([0x10; 4]), H32::from_slice(&[0x10; 4]));
    }

    #[test]
    #[should_panic]
    fn fail_too_few_elems() {
        H32::from_slice(&[0x10; 3]);
    }

    #[test]
    #[should_panic]
    fn fail_too_many_elems() {
        H32::from_slice(&[0x10; 5]);
    }
}

mod covers {
    use super::*;

    #[test]
    fn simple() {
        assert!(H32::from([0xFF; 4]).covers(&H32::zero()));
        assert!(!(H32::zero().covers(&H32::from([0xFF; 4]))));
    }

    #[test]
    fn zero_covers_zero() {
        assert!(H32::zero().covers(&H32::zero()));
    }

    #[test]
    fn ones_covers_ones() {
        assert!(H32::from([0xFF; 4]).covers(&H32::from([0xFF; 4])));
    }

    #[test]
    fn complex_covers() {
        #[rustfmt::skip]
        assert!(
            H32::from([0b0110_0101, 0b1000_0001, 0b1010_1010, 0b0110_0011]).covers(&
            H32::from([0b0010_0100, 0b1000_0001, 0b0010_1010, 0b0110_0010]))
        );
    }

    #[test]
    fn complex_uncovers() {
        #[rustfmt::skip]
        assert!(
            !(
                H32::from([0b0010_0100, 0b1000_0001, 0b0010_1010, 0b0110_0010]).covers(&
                H32::from([0b0110_0101, 0b1000_0001, 0b1010_1010, 0b0110_0011]))
            )
        );
    }
}

mod is_zero {
    use super::*;

    #[test]
    fn all_true() {
        assert!(H32::zero().is_zero());
        assert!(H64::zero().is_zero());
        assert!(H128::zero().is_zero());
        assert!(H160::zero().is_zero());
        assert!(H256::zero().is_zero());
    }

    #[test]
    fn all_false() {
        assert!(!H32::repeat_byte(42).is_zero());
        assert!(!H64::repeat_byte(42).is_zero());
        assert!(!H128::repeat_byte(42).is_zero());
        assert!(!H160::repeat_byte(42).is_zero());
        assert!(!H256::repeat_byte(42).is_zero());
    }
}

#[cfg(feature = "byteorder-support")]
mod to_low_u64 {
    use super::*;

    #[test]
    fn smaller_size() {
        assert_eq!(
            H32::from([0x01, 0x23, 0x45, 0x67]).to_low_u64_be(),
            0x0123_4567
        );
        assert_eq!(
            H32::from([0x01, 0x23, 0x45, 0x67]).to_low_u64_le(),
            0x6745_2301_0000_0000
        );
    }

    #[test]
    fn equal_size() {
        assert_eq!(
            H64::from([
                0x01, 0x23, 0x45, 0x67,
                0x89, 0xAB, 0xCD, 0xEF
            ]).to_low_u64_le(),
            0xEFCD_AB89_6745_2301
        );
        assert_eq!(
            H64::from([
                0x01, 0x23, 0x45, 0x67,
                0x89, 0xAB, 0xCD, 0xEF
            ]).to_low_u64_be(),
            0x0123_4567_89AB_CDEF
        )
    }

    #[test]
    fn larger_size() {
        assert_eq!(
            H128::from([
                0x01, 0x23, 0x45, 0x67,
                0x89, 0xAB, 0xCD, 0xEF,
                0x09, 0x08, 0x07, 0x06,
                0x05, 0x04, 0x03, 0x02
            ]).to_low_u64_be(),
            0x0908070605040302
        );
        assert_eq!(
            H128::from([
                0x01, 0x23, 0x45, 0x67,
                0x89, 0xAB, 0xCD, 0xEF,
                0x09, 0x08, 0x07, 0x06,
                0x05, 0x04, 0x03, 0x02
            ]).to_low_u64_le(),
            0x0203040506070809
        )
    }
}

#[cfg(feature = "byteorder-support")]
mod from_low_u64 {
    use super::*;

    #[test]
    fn smaller_size() {
        assert_eq!(
            H32::from_low_u64_be(0x0123_4567_89AB_CDEF),
            H32::from([0x01, 0x23, 0x45, 0x67])
        );
        assert_eq!(
            H32::from_low_u64_le(0x0123_4567_89AB_CDEF),
            H32::from([0xEF, 0xCD, 0xAB, 0x89])
        );
    }

    #[test]
    fn equal_size() {
        assert_eq!(
            H64::from_low_u64_be(0x0123_4567_89AB_CDEF),
            H64::from([
                0x01, 0x23, 0x45, 0x67,
                0x89, 0xAB, 0xCD, 0xEF
            ])
        );
        assert_eq!(
            H64::from_low_u64_le(0x0123_4567_89AB_CDEF),
            H64::from([
                0xEF, 0xCD, 0xAB, 0x89,
                0x67, 0x45, 0x23, 0x01
            ])
        )
    }

    #[test]
    fn larger_size() {
        assert_eq!(
            H128::from_low_u64_be(0x0123_4567_89AB_CDEF),
            H128::from([
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF
            ])
        );
        assert_eq!(
            H128::from_low_u64_le(0x0123_4567_89AB_CDEF),
            H128::from([
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01
            ])
        )
    }
}
