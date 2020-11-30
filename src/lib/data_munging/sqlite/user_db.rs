// this example uses the users.db sqlite3 database file in the
// testdata/ directory; run the init script to recreate this
// db file (with some preloaded rows)

#[allow(unused_imports)]
use bcrypt::{hash, verify, BcryptError};
#[allow(unused_imports)]
use sqlite::Error as SqErr;
#[allow(unused_imports)]
use std::time::{SystemTime, UNIX_EPOCH};

// an error type to aggregate internal (3rd party) error types
#[derive(Debug)]
enum UBaseErr {
    DbErr(SqErr),
    HashError(BcryptError),
}

// to implicitly convert the internal error types to my own
// error type, so that ? can work seamlessly
impl From<SqErr> for UBaseErr {
    fn from(err: SqErr) -> Self {
        UBaseErr::DbErr(err)
    }
}

impl From<BcryptError> for UBaseErr {
    fn from(err: BcryptError) -> Self {
        UBaseErr::HashError(err)
    }
}

struct UserBase {
    fname: String,
}

#[allow(dead_code)]
impl UserBase {
    fn add_user(&self, u_name: &str, p_word: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(p_word, 7)?;
        let mut st = conn.prepare("insert into users (u_name, p_word) values (?, ?);")?;
        st.bind(1, u_name)?;
        st.bind(2, &hpass as &str)?;
        st.next()?;
        Ok(())
    }
}

#[allow(dead_code)]
fn get_current_micro() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_micros()
}

#[test]
fn demo_add_new_user() {
    // db file does not exist (expect SqErr implicitly converted
    // to DbErr of own type)
    {
        let ub = UserBase {
            fname: "/asd".to_string(),
        };
        assert_eq!(ub.add_user("archville", "e2m1").is_err(), true);
    }
    // invalid plaintext, expect HashErr
    {
        let ub = UserBase {
            fname: "/tmp/rw_rust_testdata/users.db".to_string(),
        };
        let o = ub.add_user("archville", "\0");
        assert_eq!(o.is_err(), true);
        match o {
            Err(UBaseErr::HashError(_)) => (),
            _ => panic!("wrong error type! {:?}", o),
        }
    }

    // when everything works;
    // but it will modify the db file
    // {
    //     let ub = UserBase {
    //         fname: "/tmp/rw_rust_testdata/users.db".to_string(),
    //     };
    //     let n = format!("arch_{}", get_current_micro());
    //     let o = ub.add_user(
    //         &n as &str, 
    //         "episode 8"
    //     );
    //     assert_eq!(o.is_ok(), true);
    // }
}
