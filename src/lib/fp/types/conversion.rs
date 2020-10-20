// rust std lib cookbook P/158
// type conversion by traits

#[allow(unused_imports)]
use std::ops::MulAssign;

#[test]
fn demo_() {
    #[derive(Debug)]
    struct DoubleVec<T>(Vec<T>);

    impl<T> From<Vec<T>> for DoubleVec<T>
    where
    T: MulAssign<i32>,
    {
        fn from(mut vec: Vec<T>) -> Self {
            // for elem in &mut vec {
            //     *elem *= 2;
            // }
            vec.iter_mut().for_each(|elem| *elem *= 2);
            DoubleVec(vec)
        }
    }

    // test the conversion from vec to double vec
    {
        let src = vec![1, 2, 3];
        let dest = DoubleVec::from(src);
        let dump = format!("{:?}", dest);
        assert_eq!("DoubleVec([2, 4, 6])", dump);
    }
}

