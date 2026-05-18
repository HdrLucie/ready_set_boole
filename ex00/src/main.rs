fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        // XOR : 95 ^ 10 -> 1011111 ^ 0001010 = 1010101 -> 85.
        // XOR operation returns 1 if the inputs are different,
        // and 0 if they are the same.
        let sum = a ^ b;

        // AND : 95 & 10 -> 1011111 & 0001010 = 0001010 -> 10 -> 10 << 1 -> 0010100 : 20.
        // The AND operation returns 1 (true) if the both the inputs are 1,
        // and 0 (false) otherwise.
        // And we use the left shift in order to change the position of the carry.
        let carry = (a & b) << 1;
        a = sum;
        b = carry;
    }
    a
}

fn main() {
    println!("95 + 10 = {}", adder(95, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(10, adder(5, 5));
        assert_eq!(105, adder(95, 10));
        assert_eq!(59604, adder(6923, 52681));
        assert_eq!(0, adder(0, 0));
        assert_eq!(1, adder(0, 1));
        assert_eq!(2, adder(1, 1));
        assert_eq!(999, adder(500, 499));
        assert_eq!(0, adder(1, u32::MAX));
        // Wrong test
        assert_eq!(10, adder(6, 3));
    }
}
