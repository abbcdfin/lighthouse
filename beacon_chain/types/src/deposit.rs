use super::ssz::{Decodable, DecodeError, Encodable, SszStream};
use super::{DepositData, Hash256};
use crate::test_utils::TestRandom;
use rand::RngCore;

#[derive(Debug, PartialEq, Clone)]
pub struct Deposit {
    pub merkle_branch: Vec<Hash256>,
    pub merkle_tree_index: u64,
    pub deposit_data: DepositData,
}

impl Encodable for Deposit {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append_vec(&self.merkle_branch);
        s.append(&self.merkle_tree_index);
        s.append(&self.deposit_data);
    }
}

impl Decodable for Deposit {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (merkle_branch, i) = <_>::ssz_decode(bytes, i)?;
        let (merkle_tree_index, i) = <_>::ssz_decode(bytes, i)?;
        let (deposit_data, i) = <_>::ssz_decode(bytes, i)?;

        Ok((
            Self {
                merkle_branch,
                merkle_tree_index,
                deposit_data,
            },
            i,
        ))
    }
}

impl<T: RngCore> TestRandom<T> for Deposit {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            merkle_branch: <_>::random_for_test(rng),
            merkle_tree_index: <_>::random_for_test(rng),
            deposit_data: <_>::random_for_test(rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ssz::ssz_encode;
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = Deposit::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);
        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }
}
