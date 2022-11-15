fn vector_of_digits_in_base(base: i32, number: i32) -> Vec<i32> {
    let mut vector = vec![];
    let mut number = number;
    while number >= base {
        vector.push(number % base);
        number /= base;
    }
    vector.push(number);
    vector.reverse();
    vector
}

#[cfg(test)]
mod test {
    use super::vector_of_digits_in_base;

    #[test]
    fn simple_base_10() {
        let n = 1234;
        assert_eq!(vec![1, 2, 3, 4], vector_of_digits_in_base(10, n))
    }

    #[test]
    fn simple_base_16() {
        let n = 0xfa28ab;
        assert_eq!(
            vec![0xf, 0xa, 0x2, 0x8, 0xa, 0xb],
            vector_of_digits_in_base(16, n)
        )
    }
}
