#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let aliquot_sum: u64 = (1..num).filter(|&x| num % x == 0).sum();

    if aliquot_sum == num {
        Some(Classification::Perfect)
    } else if aliquot_sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_number() {
        assert_eq!(classify(6), Some(Classification::Perfect));
        assert_eq!(classify(28), Some(Classification::Perfect));
    }

    #[test]
    fn test_abundant_number() {
        assert_eq!(classify(12), Some(Classification::Abundant));
        assert_eq!(classify(24), Some(Classification::Abundant));
    }

    #[test]
    fn test_deficient_number() {
        assert_eq!(classify(8), Some(Classification::Deficient));
        assert_eq!(classify(7), Some(Classification::Deficient));
    }

    #[test]
    fn test_zero() {
        assert_eq!(classify(0), None);
    }
}
