// rust std lib cookbook P/146

// https://docs.rs/bitflags/1.2.1/bitflags/

extern crate bitflags;

#[allow(unused_imports)]
use bitflags::bitflags;

#[test]
fn demo_bitflags_as_struct() {
    bitflags! {
        struct AddOns: u32 {
            const RED_DOT    = 0b0000_0001;
            const SUPPRESSOR = 0b0000_0010;
            const FRONT_GRIP = 0b0000_0100;
            const EXTRA_MAG  = 0b0000_1000;
            const ALL = Self::RED_DOT.bits
                      | Self::SUPPRESSOR.bits
                      | Self::FRONT_GRIP.bits
                      | Self::EXTRA_MAG.bits;
        }
    }

    impl AddOns {
        // implementing a "clear" method
        fn clear(&mut self) -> &mut Self {
            self.bits = 0;
            self
        }
    }

    let basic = AddOns::RED_DOT;
    let recon = AddOns::RED_DOT | AddOns::SUPPRESSOR;

    assert!(basic.contains(AddOns::RED_DOT));
    assert!(!recon.contains(AddOns::EXTRA_MAG));

    let mut empty = basic;
    empty.clear();
    assert!(!empty.contains(AddOns::RED_DOT));
}
