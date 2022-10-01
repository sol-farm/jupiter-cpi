anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph");

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Side::Ask => match other {
                Side::Ask => true,
                _ => false,
            },
            Side::Bid => match other {
                Side::Bid => true,
                _ => false,
            },
        }
    }
    fn ne(&self, other: &Self) -> bool {
        match self {
            Side::Ask => match other {
                Side::Bid => true,
                _ => false,
            },
            Side::Bid => match other {
                Side::Ask => true,
                _ => false,
            },
        }
    }
}