#[test]
fn demo_define_lambda() {
    // f is of type fn(i32) -> i32 (lowercase fn)
    let f = |x: i32| -> i32 {
        // some internal effects 
        let _x = vec![1, 2];
        x + 1
     };
     assert_eq!(2, f(1));

     // use type annotation
     let g: fn(i32) -> i32 = |x| x + 1;
     assert_eq!(2, g(1));

     // use lambda as first-class value
     let fns: Vec<fn(i32)-> i32> = vec![
         |x| x + 1,
         |x| x * 2,
         |x| x - 10,
         |x| x % 4,
     ];
     let mut x: i32 = 1;
     for f in fns.iter() {
        x = f(x);
     }
     assert_eq!(-2, x);
}

////////////////////////////////////////////////////////////////
// I can not define a generic closure that takes type parameters
//
// make it a function instead
////////////////////////////////////////////////////////////////
