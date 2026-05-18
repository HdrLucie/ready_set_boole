fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let sum = a ^ b;
        let carry = (a & b) << 1;
        a = sum;
        b = carry;
    }
    a
}

// It's called the Russian peasant method : 
// Instead of directly multiplying a and b,
// we repeatedly halve b and double a, 
// leveraging the fact that multiplication can be rewritten as repeated addition.
// a : 2 | b : 12
// a : 4 | b : 6
// a : 8 | b : 3
// a : 16 | b : 1
// a : 32 | b : 0
fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut result = 0;

    while b > 0 {
        if (b & 1) != 0 {
            result = adder(result, a);
        }
        a <<= 1;
        b >>= 1;
    }
    result
}

fn main() {
    println!("{}", multiplier(2, 13));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
        assert_eq!(144, multiplier(12, 12));
        assert_eq!(30, multiplier(6, 5));
        assert_eq!(47448, multiplier(72, 659));
        assert_eq!(0, multiplier(384, 0));
    }
}
