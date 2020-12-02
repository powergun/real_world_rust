
#[macro_export]
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("function {:?} is called", stringify!($fn_name));
        }
    }
}

#[test]
fn demo_macro_build_fn_use_indentifier() {
    {
        build_fn!(e1m1);

        e1m1();
    }
    
    // function e1m1() is not accessible in the outer scope.
    // e1m1();
}
