// source:
// rust std cookbook P/11
#[test]
fn demo_string_concatenation() {
    // by moving s1 into a new variable
    // Pros:
    // - is memory efficient
    // - is readable
    // Cons:
    // - lose the variable on LHS
    {
        // "e1m1" is a string slice (&str)
        let s1 = "e1m1".to_string();
        let s2 = "iddqd";
        let s12 = s1 + s2;
        assert_eq!("e1m1iddqd".to_string(), s12);

        // s1 can not be used anymore
        // assert_eq!("e1m1", s1);
        // s2 is still alive
        assert_eq!("iddqd", s2);
    }

    // by cloning
    {
        let s1 = "e1m1".to_string();
        let s2 = "idkfa";
        let ss = s1.clone() + s2;
        assert_eq!("e1m1idkfa".to_string(), ss);
        assert_eq!("e1m1".to_string(), s1);
        assert_eq!("idkfa", s2);
    }

    // by mutating
    {
        let mut s1 = "e1m1".to_string();
        let s2 = "idkfa";
        s1.push_str(s2);
        assert_eq!("e1m1idkfa", s1);
        assert_eq!("idkfa", s2);
    }
}

#[derive(Debug)]
struct Log {
    pub name: String,
    pub length: u64,
}

// source:
// rust std cookbook P/13
#[test]
fn demo_string_concatenation_by_format_macro() {
    {
        let s1 = "iddqd";
        let ss = format!("something {}", s1);
        assert_eq!("something iddqd", ss);
    }

    // concat any data types that implement the Display trait
    // like in DAML I can <> values that are conforming to Show
    // type class
    {
        let ss = format!("there is {} {}", "a cow", 123);
        assert_eq!("there is a cow 123", ss);
    }

    // include certain (positional) param multiple times into
    // the string
    {
        let ss = format!("there {0} {0} {0}", 1);
        assert_eq!("there 1 1 1", ss);
    }

    // use keyword parameters
    {
        let ss = format!("there {is} {a} {cow}", is = "is", a = "a", cow = "cow",);
        assert_eq!("there is a cow", ss);
    }

    // use struct that derives debug strait
    // also see P/14 about Display trait
    {
        let l = Log {
            name: "A".to_string(),
            length: 1,
        };
        let ss = format!("{:?}", l);
        assert_eq!("Log { name: \"A\", length: 1 }", ss);
    }
}

// split
// source
// rust std lib cookbook P/54
#[test]
fn demo_split_at() {
    let (left, right) = "there is cow".split_at(5);
    assert_eq!("there", left);
    assert_eq!(" is cow", right)
}

#[test]
fn demo_split_by_separator() {
    // to use collect() method on an iter, I must specify the
    // collection's type
    let substrings: Vec<&str> = "there is a cow".split(" ").collect();
    assert_eq!(4, substrings.len());
    assert_eq!(vec!["there", "is", "a", "cow"], substrings);

    // but I can tell the compiler to deduce the element type
    let ss: Vec<_> = "there is a cow".split(" ").collect();
    assert_eq!(ss, substrings);

    // P/55
    // observe the empty tail substring and how to eliminate it
    // using a special split function
    {
        let text = "let a = 1; a += 1;";
        let ss1: Vec<_> = text.split(";").collect();
        assert_eq!("", ss1[2]);

        let ss2: Vec<_> = text.split_terminator(";").collect();
        assert_eq!(2, ss2.len());
    }

    // special separators
    {
        let text = "let a = 1; a += 1;";
        assert_eq!(3, text.split(char::is_numeric).count());
        assert_eq!(7, text.split(char::is_whitespace).count());
    }

    // split by whitespace
    {
        let text = r"there
        is a 
        cow";
        assert_eq!(4, text.split_whitespace().count());
    }
}
