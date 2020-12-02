#[macro_export]
macro_rules! print_vec {
    () => {
        println!("empty!");
    };

    // use () instead of { {} } 
    // $(...)* is to repeat ... expression until the match
    // case no longer applies 
    ($($x:expr),*) => (
        $(println!("macro: {:?}", $x);)*
    );
}

#[test]
fn demo_custom_vector_macro() {
    print_vec![1, 1, 2, 3, 4];
    print_vec![];
}

#[macro_export]
macro_rules! single_param_unit {
    ($x: expr) => {
        println!("macro: {}", $x);
    };
}

#[macro_export]
macro_rules! single_param {
    ($x: expr) => {
        $x
    }
}

#[test]
fn demo_single_param_macro() {
    // this macro returns ()
    single_param_unit!(1);

    // these won't compile
    // single_param_unit!();
    // single_param_unit!(1, 2, 3);

    // returns i32, same parameter constraint
    let x: i32 = single_param!(1);
    assert_eq!(x, 1);
}

#[macro_export]
macro_rules! head_tail {
    // this is similar to [] list notation in Haskell;
    // to match an expression with no element
    () => {

    };
    // this is similar to (hd::tl) notation in Haskell;
    // the entire expression must consist of at least one
    // head element (hd), and optionally, any number of tail
    // elements (tl);
    ($($x: expr), *) => {

    };
}

#[test]
fn demo_multiple_param_macro() {
    // it does nothing
    head_tail!();
    head_tail!(1);
    head_tail!(1, 2, 3, 3);
}

#[macro_export]
macro_rules! named_xy {
    (x => $e: expr) => {
        println!("x is {:?}", $e);
    };
    (y => $e: expr) => {
        println!("y is {:?}", $e);
    };
}

#[test]
fn demo_named_param() {
    // these "params" are expressions, meaning that they can
    // be 1 * 2 or even a closure
    // this reminds me Scala's concept of everything is an
    // expression
    named_xy!(x => 12);
    named_xy!(y => vec![1, 2, 3]);
    named_xy!(x => {
        println!("some effect");
        1
    });
}

// the "type" in the macros can be:
// expr
// ident (identifier)
// block
// stmt
// pat
// path
// meta
// ty
// tt