use num_bigint::BigInt;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Money {
    pub amount: BigInt,
}

impl Money {
    // Functions

    pub fn new(amount: BigInt) -> Self {
        Self { amount }
    }

    pub fn of(value: i128) -> Self {
        Self {
            amount: BigInt::from(value),
        }
    }

    pub fn add(a: &Money, b: &Money) -> Money {
        Money {
            amount: &a.amount + &b.amount,
        }
    }

    pub fn substract(a: &Money, b: &Money) -> Money {
        Money {
            amount: &a.amount - &b.amount,
        }
    }

    // Methods

    pub fn is_positive_or_zero(&self) -> bool {
        self.amount >= BigInt::from(0 as i128)
    }

    pub fn is_negative(&self) -> bool {
        self.amount < BigInt::from(0 as i128)
    }

    pub fn is_positive(&self) -> bool {
        self.amount > BigInt::from(0 as i128)
    }

    pub fn is_greater_than_or_equal_to(&self, money: &Money) -> bool {
        self.amount >= money.amount
    }

    pub fn is_greater_than(&self, money: &Money) -> bool {
        self.amount > money.amount
    }

    pub fn minus(&self, money: &Money) -> Self {
        Self {
            amount: &self.amount - &money.amount,
        }
    }

    pub fn plus(&self, money: &Money) -> Self {
        Self {
            amount: &self.amount + &money.amount,
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            amount: -&self.amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of() {
        let money = Money::of(42);
        assert_eq!(BigInt::from(42 as i128), money.amount);
    }

    #[test]
    fn test_add() {
        let a = Money::of(1);
        let b = Money::of(2);
        assert_eq!(BigInt::from(3 as i128), Money::add(&a, &b).amount);
    }

    #[test]
    fn test_is_positive_or_zero() {
        let minus = Money::of(-1);
        assert_eq!(false, minus.is_positive_or_zero());

        let zero = Money::of(0);
        assert_eq!(true, zero.is_positive_or_zero());

        let one = Money::of(1);
        assert_eq!(true, one.is_positive_or_zero());
    }

    #[test]
    fn test_is_greater_than_or_equal_to() {
        let minus_two = Money::of(-2);
        let minus_one = Money::of(-1);
        let zero = Money::of(0);
        let one = Money::of(1);
        let two = Money::of(2);

        assert_eq!(true, minus_one.is_greater_than_or_equal_to(&minus_two));
        assert_eq!(true, zero.is_greater_than_or_equal_to(&minus_one));
        assert_eq!(true, one.is_greater_than_or_equal_to(&minus_one));
        assert_eq!(true, one.is_greater_than_or_equal_to(&zero));
        assert_eq!(true, two.is_greater_than_or_equal_to(&one));

        assert_eq!(true, minus_one.is_greater_than_or_equal_to(&minus_one));
        assert_eq!(true, zero.is_greater_than_or_equal_to(&zero));
        assert_eq!(true, one.is_greater_than_or_equal_to(&one));

        assert_eq!(false, minus_two.is_greater_than_or_equal_to(&minus_one));
        assert_eq!(false, minus_one.is_greater_than_or_equal_to(&zero));
        assert_eq!(false, zero.is_greater_than_or_equal_to(&one));
        assert_eq!(false, one.is_greater_than_or_equal_to(&two));
    }

    #[test]
    fn test_plus() {
        let a = Money::of(1);
        let b = Money::of(2);
        assert_eq!(BigInt::from(3 as i128), a.plus(&b).amount);
    }

    #[test]
    fn test_negate() {
        let money = Money::of(1);
        let money_negated = Money::of(-1);
        assert_eq!(money_negated, money.negate());
    }
}
