#[allow(unused_imports)]
use std::cmp::PartialEq;

#[derive(Debug)]
struct USD(i32);
// USD is a named tuple;
// to access its wrapped value of i32, use .0 member
#[derive(Debug, PartialEq)]
struct RMB(i32);

trait ToUSD {
    fn to_usd(&self) -> USD;

    // convert() bridges unrelated types;
    // any two types that impl FromUSD & ToUSD can be converted
    // to and from each other
    fn convert<T: FromUSD>(&self) -> T {
        T::from_usd(self.to_usd())
    }
}

trait FromUSD {
    fn from_usd(u: USD) -> Self;
}

impl FromUSD for String {
    fn from_usd(u: USD) -> Self {
        u.0.to_string()
    }
}

impl FromUSD for RMB {
    fn from_usd(u: USD) -> Self {
        RMB(u.0)
    }
}

impl ToUSD for String {
    fn to_usd(&self) -> USD {
        USD(1)
    }
}

impl ToUSD for RMB {
    fn to_usd(&self) -> USD {
        USD(self.0)
    }
}

#[test]
fn demo_type_constraint() {
    let sss: String = "$$".to_string();
    let rmb: RMB = sss.convert();
    assert_eq!(rmb, RMB(1));
}
