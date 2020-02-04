use curve25519_dalek::scalar::Scalar;

pub(crate) fn pad<T>(messages: &[T], width: usize) -> Vec<T>
where
    T: From<Scalar> + Clone,
{
    let length = messages.len() + 1;
    let arity = width - 1;
    let offset = ((length % arity) != 0) as usize;
    let size = (length / arity + offset) * width;

    let zero: T = Scalar::zero().into();
    let one: T = Scalar::one().into();
    let mut words = vec![zero; size];
    let mut messages = messages.iter();

    for chunk in words.chunks_mut(width) {
        for elem in chunk.iter_mut().skip(1) {
            if let Some(message) = messages.next() {
                *elem = message.clone();
            } else {
                *elem = one;
                return words;
            }
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;
    use bulletproofs::r1cs::LinearCombination;

    #[test]
    fn test_scalar_padding_width_3() {
        let zero = Scalar::zero();
        let one = Scalar::one();
        let two = Scalar::from(2u64);
        let three = Scalar::from(3u64);
        let four = Scalar::from(4u64);

        assert_eq!(&pad(&[two], 3), &[zero, two, one]);
        assert_eq!(&pad(&[two, three], 3), &[zero, two, three, zero, one, zero]);
        assert_eq!(
            &pad(&[two, three, four], 3),
            &[zero, two, three, zero, four, one]
        );
    }

    #[test]
    fn test_scalar_padding_width_4() {
        let zero = Scalar::zero();
        let one = Scalar::one();
        let two = Scalar::from(2u64);
        let three = Scalar::from(3u64);
        let four = Scalar::from(4u64);

        assert_eq!(&pad(&[two], 4), &[zero, two, one, zero]);
        assert_eq!(&pad(&[two, three], 4), &[zero, two, three, one]);
        assert_eq!(
            &pad(&[two, three, four], 4),
            &[zero, two, three, four, zero, one, zero, zero]
        );
    }

    #[test]
    fn test_lc_padding_width_3() {
        let zero = LinearCombination::from(Scalar::zero());
        let one = LinearCombination::from(Scalar::one());
        let two = LinearCombination::from(Scalar::from(2u64));
        let three = LinearCombination::from(Scalar::from(3u64));
        let four = LinearCombination::from(Scalar::from(4u64));

        assert_eq!(
            &pad(&[two.clone()], 3),
            &[zero.clone(), two.clone(), one.clone()]
        );
        assert_eq!(
            &pad(&[two.clone(), three.clone()], 3),
            &[
                zero.clone(),
                two.clone(),
                three.clone(),
                zero.clone(),
                one.clone(),
                zero.clone()
            ]
        );
        assert_eq!(
            &pad(&[two.clone(), three.clone(), four.clone()], 3),
            &[
                zero.clone(),
                two.clone(),
                three.clone(),
                zero.clone(),
                four.clone(),
                one.clone()
            ]
        );
    }
}