// source
// rust std cookbook P/30

extern crate rand;

// the random generator by default uses uniform dist;
// I can specify a particular dist at the creation time

#[test]
fn demo_rand_numbers() {
    // will be any int32 between std::i32::MIN and MAX
    // the initialization of r1 and r2 is equivalent
    let r1 = rand::random::<i32>();
    let r2: i32 = rand::random();
    assert_ne!(r2, r1);

    // every primitive data type can be randomized
    let r_char: char = rand::random();
    assert_ne!('\0', r_char);
}

#[test]
fn demo_use_generator() {
    // see: https://docs.rs/rand/0.6.5/rand/fn.thread_rng.html
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let x: bool = rng.gen();
    println!("{}", x);

    // y is [0, 10)
    let y = rng.gen_range(0, 10);
    assert_eq!(true, y >= 0);

    // z is [0, 1.0)
    let z = rng.gen_range(0.0, 1.0);
    assert_eq!(true, z >= 0.0);
}
