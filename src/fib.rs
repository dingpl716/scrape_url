#[cfg(test)]
mod fib {
    fn next_fib(a: u32, b: u32, i: u8) -> (u32, u32, u8) {
        let c = a + b;
        return (b, c, i + 1);
    }

    fn fib_loop(n: u8) -> u32 {
        let mut a = 1;
        let mut b = 1;
        let mut i = 2u8;

        loop {
            (a, b, i) = next_fib(a, b, i);

            println!("next value is {}", b);

            if i >= n {
                return b;
            }
        }
    }

    fn fib_while(n: u8) -> u32 {
        let (mut a, mut b, mut i) = (1, 1, 2);

        while i < n {
            (a, b, i) = next_fib(a, b, i);

            println!("next value is {}", b);
        }

        return b;
    }

    fn fib_for(n: u8) -> u32 {
        let (mut a, mut b) = (1, 1);
        for _i in 2..n {
            (a, b, _) = next_fib(a, b, 0);
            println!("next value is {}", b);
        }

        return b;
    }

    pub fn run() {
        let n = 10;
        println!("The final one is: {}", fib_loop(n));
        println!("The final one is: {}", fib_while(n));
        println!("The final one is: {}", fib_for(n));

        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = &arr[..11];
        println!("{:?}", b);
    }

    #[test]
    fn test_fib() {
        let n = 10;
        assert_eq!(fib_loop(n), 55);
        assert_eq!(fib_while(n), 55);
        assert_eq!(fib_for(n), 55);
    }
}
