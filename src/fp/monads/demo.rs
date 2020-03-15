// data T f g a
// ^^^^^^^^^^ these are the structure
pub trait Monad<A> {
    fn return_(x: A) -> Self;
    fn bind_<MB, B, F>(&self, f: F) -> MB
    where
        F: Fn(A) -> MB,
        MB: Monad<B>;
}

pub enum Maybe<T> {
    Just { x: T },
    Nothing,
}

// I struggled to define the trait-impl, but this helped me:
// https://users.rust-lang.org/t/beginner-query-implementing-traits-for-generic-structs/2871/3
// NOTE: implementing the idiomatic functor and monad in rust is
//       difficult, see the explanation and workaround here
// https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html
//
// the implementation requires a higher-kind-of Self which is not
// a concept Rust code uses often
// impl<A> Monad<A> for Maybe<A> {
//     fn return_(x: A) -> Self {
//         Maybe::Just { x: x }
//     }
//     fn bind_<MB, B, F>(&self, f: F) -> MB
//     where
//         F: Fn(A) -> MB,
//         MB: Monad<B>,
//     {
//         Maybe::Nothing
//     }
// }

#[test]
pub fn test_monad_return() {}
