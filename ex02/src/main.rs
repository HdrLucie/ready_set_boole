fn gray_code(n: u32) -> u32 {
    let res = n ^ (n >> 1);
    res
}

fn main() {
    println!("res : {}", gray_code(0));
}

#[cfg(test)]
mod tests {
    use super::*;
        #[test]
        fn test_gray_code() {
            assert_eq!(0, gray_code(0));
            assert_eq!(1, gray_code(1));
            assert_eq!(3, gray_code(2));
            assert_eq!(2, gray_code(3));
            assert_eq!(6, gray_code(4));
            assert_eq!(7, gray_code(5));
            assert_eq!(5, gray_code(6));
            assert_eq!(4, gray_code(7));
            assert_eq!(12, gray_code(8));
            assert_eq!(46, gray_code(52));
            assert_eq!(236, gray_code(183));
            assert_eq!(19052, gray_code(29623));
            assert_eq!(608878, gray_code(947124));
        }
}
