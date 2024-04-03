use crate::mt19937::{untemper, MT19937};

pub fn crack_mt19937(rng: &mut MT19937) -> MT19937 {
    let mut mt = vec![];
    for _ in 0..624 {
        let val = rng.gen_u32();
        mt.push(untemper(val));
    }
    MT19937::new_with_mt(mt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crack_mt19937() {
        let mut rng = MT19937::new(rand::random::<u32>());
        let mut new_rng = crack_mt19937(&mut rng);

        for _ in 0..624 {
            let v1 = rng.gen_u32();
            let v2 = new_rng.gen_u32();
            assert_eq!(v1, v2);
        }
    }
}
