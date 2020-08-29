#[macro_export]
macro_rules! print_vec {
    () => {
        println!("empty!");
    };
    ( $( $x:expr),* ) => {
        {
            $( println!("macro: {:?}", $x); )*
        }
    };

}

#[test]
fn demo_custom_vector_macro() {
    print_vec![1, 1, 2, 3, 4];
    print_vec![];
}
