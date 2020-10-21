// rust std lib cookbook P/158
// type conversion by traits

#[allow(unused_imports)]
use std::ops::MulAssign;

#[test]
fn demo_convert_vec_to_double_vec() {
    #[derive(Debug)]
    struct DoubleVec<T>(Vec<T>);

    // P/161
    // implementing the From trait means defining how to obtain
    // a type from another
    impl<T> From<Vec<T>> for DoubleVec<T>
    where
    T: MulAssign<i32>,  // T implements *= operator
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

// P161
// it is considered good practice to extend all of your trait
// definitions that work with vectors (Vec<T>) to also work with
// slices (&[T]).
// this way you gain generality and performance, as you can
// operate on direct references to array (such as &[1, 2, 3])
// and ranges of other vectors (vec[1..3]) without converting
// them first
// the best practice carries over to functions as well, where
// you should always accept a slice of your type if possible
#[test]
fn demo_convert_slice_to_double_vec() {
    #[derive(Debug)]
    struct DoubleVec<T>(Vec<T>);

    impl<T> From<Vec<T>> for DoubleVec<T>
    where
    T: MulAssign<i32>,
    {
        fn from(mut vec: Vec<T>) -> Self {
            vec.iter_mut().for_each(|elem| *elem *= 2);
            DoubleVec(vec)
        }
    }

    // allowing conversion from a slice of Ts
    impl<'a, T> From<&'a [T]> for DoubleVec<T>
    where
    T: MulAssign<i32> + Clone,
    {
        fn from(slice: &[T]) -> Self {
            // Vec<T: MulAssign<i32>> automatically implements
            // Into<DoubleVec<T>>

            // .to_vec() converts a slice to a vector 
            // it requires T to be copy-able, hence requiring
            // the Clone trait

            // .into() is powered by Into trait, the opposite
            // of From trait
            // every T that implements From for U, automatically
            // lets U implement Into for T
            slice.to_vec().into()
        }
    }

    // test the conversion from vec to double vec
    {
        let src = vec![1, 2, 3];

        // the input is a slice of i32 that points to `src` vec;
        // it only contains the [1] element
        // see: https://doc.rust-lang.org/std/slice/
        let dest: DoubleVec<_> = DoubleVec::from(&src[1..2]);
        let dump = format!("{:?}", dest);
        assert_eq!("DoubleVec([4])", dump);
    }
}

#[test]
fn demo_convert_ref_to_double_vec_to_ref_to_vec() {
    #[derive(Debug)]
    struct DoubleVec<T>(Vec<T>);

    impl<T> From<Vec<T>> for DoubleVec<T>
    where
    T: MulAssign<i32>,
    {
        fn from(mut vec: Vec<T>) -> Self {
            vec.iter_mut().for_each(|elem| *elem *= 2);
            DoubleVec(vec)
        }
    }

    // allow conversion from a &DoubleVec<T> to a &Vec<T>
    impl<T> AsRef<Vec<T>> for DoubleVec<T> {
        fn as_ref(&self) -> &Vec<T> {
            &self.0
        }
    }

    // a reference to double vec can be converted to a ref
    // to vec, which in turn dereferenced to a slice
    {
        let dest = DoubleVec::from(vec![1, 2, 3]);
        let dump = format!("{:?}", dest.as_ref());
        assert_eq!("[2, 4, 6]", dump);
    }
}
