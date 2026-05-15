// Teardown Ind IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Teardown Ind IE TV

pub const TEARDOWN_IND: u8 = 19;
pub const TEARDOWN_IND_LENGTH: u16 = 1;

// Teardown Ind IE implementation
//
// Octet 2: LSB is Teardown Ind, upper 7 bits are spare. Encoding sets the
// spare bits to '1' per Figure 24; decoding treats them as don't-care.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeardownInd {
    pub t: u8,
    pub teardown: bool, // Teardown Ind
}

impl Default for TeardownInd {
    fn default() -> TeardownInd {
        TeardownInd {
            t: TEARDOWN_IND,
            teardown: true,
        }
    }
}

impl IEs for TeardownInd {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        match self.teardown {
            false => buffer.push(0xfe),
            true => buffer.push(0xff),
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= (TEARDOWN_IND_LENGTH + 1) as usize {
            let data = TeardownInd {
                teardown: (buffer[1] & 0x01) != 0,
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        TEARDOWN_IND_LENGTH as usize + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn teardown_ind_ie_unmarshal_test() {
    let encoded_ie: [u8; 2] = [0x13, 0xff];
    let test_struct = TeardownInd {
        t: TEARDOWN_IND,
        teardown: true,
    };
    let i = TeardownInd::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn teardown_ind_ie_marshal_test() {
    let encoded_ie: [u8; 2] = [0x13, 0xfe];
    let test_struct = TeardownInd {
        t: TEARDOWN_IND,
        teardown: false,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

// Spare bits are ignored on decode; only the LSB determines the value.
#[test]
fn teardown_ind_ie_unmarshal_spare_bits_ignored_test() {
    for (octet, expected) in [(0x00, false), (0x01, true), (0xaa, false), (0x55, true)] {
        let encoded_ie: [u8; 2] = [0x13, octet];
        let decoded = TeardownInd::unmarshal(&encoded_ie).unwrap();
        assert_eq!(decoded.teardown, expected, "octet 0x{:02x}", octet);
    }
}
