// FP in rust L733

cached! {
    FIB;
    fn fib(n: u64) -> u64 = {
        if n==0 || n==1 { return n }
        fib(n-1) + fib(n-2)
    }
}

#[test]
pub fn test_fib_expect_fast() {
    println!("{}", fib(20));
    println!("{}", fib(21));
}
