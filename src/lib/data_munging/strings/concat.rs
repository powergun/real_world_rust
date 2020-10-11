
// source:
// rust std cookbook P/11
#[test]
fn demo_string_concat() {
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
fn demo_string_concat_by_format_macro() {
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
        let ss = format!(
            "there {is} {a} {cow}",
            is="is",
            a="a",
            cow="cow",
        );
        assert_eq!("there is a cow", ss);
    }

    // use struct that derives debug strait
    // also see P/14 about Display trait
    {   
        let l = Log{name:"A".to_string(), length:1};
        let ss = format!("{:?}", l);
        assert_eq!("Log { name: \"A\", length: 1 }", ss);
    }
}
