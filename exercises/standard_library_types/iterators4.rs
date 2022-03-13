// iterators4.rs

struct Factorial {
    num: u64,
}

impl Factorial {
    fn new(num: u64) -> Factorial {
        Factorial { num: num }
    }
}

impl Iterator for Factorial {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.num {
            0 => None,
            default => {
                let val = self.num;
                self.num -= 1;
                Some(val)
            }
        }
    }
}

pub fn factorial(num: u64) -> u64 {
    let fact = Factorial::new(num);
    fact.product::<u64>()
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
