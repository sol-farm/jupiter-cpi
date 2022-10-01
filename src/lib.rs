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

impl Side {
    /// returns true if the Side variant is an a => b swap
    pub fn a_to_b(&self) -> bool {
        self.eq(&Side::Ask)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_side() {
        assert!(Side::Ask.eq(&Side::Ask));
        assert!(Side::Bid.eq(&Side::Bid));
        assert!(Side::Ask.ne(&Side::Bid));
        assert!(Side::Bid.ne(&Side::Ask));
        assert!(!Side::Ask.eq(&Side::Bid));
        assert!(!Side::Bid.eq(&Side::Ask));
        assert!(!Side::Bid.a_to_b());
        assert!(Side::Ask.a_to_b());
    }
}
