// this example uses the users.db sqlite3 database file in the
// testdata/ directory; run the init script `init_sqlite3_users_db.sh*`
// to recreate this db file (with some preloaded rows)

// it shows how to use a good password enc library to store the
// hashed user password (with salt)

// Note how this example resembles the daml/haskell idiom:
// - use some kind of IO monad to talk to the real world
// - use custom sum type to represent errors and exceptional conditions
// - the ? notation can be loosely considered monad bind
//

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
    UserNotFound,
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

    fn pay(&self, u_from: &str, u_to: &str, amount: i64) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(
            r#"
            insert into transactions
                (u_from, u_to, t_date, t_amount)
            values
                ( ? , ? , datetime("now") , ? )
            ;
        "#,
        )?;
        st.bind(1, u_from)?;
        st.bind(2, u_to)?;
        st.bind(3, amount)?;
        st.next()?;
        Ok(())
    }

    fn validate_user(&self, u_name: &str, p_word: &str) -> Result<bool, UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(
            r#"
            select p_word from users
                where u_name = ?
            ;
        "#,
        )?;
        st.bind(1, u_name)?;
        match st.next() {
            // 0 is the element index;
            // use turbo fish to set the resulting type;
            Ok(sqlite::State::Row) => {
                let p_hash = st.read::<String>(0)?;
                let verified = bcrypt::verify(p_word, &p_hash).unwrap_or(false);
                Ok(verified)
            }
            _ => Err(UBaseErr::UserNotFound),
        }
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
fn demo_add_new_user_and_payment() {
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
    {
        let ub = UserBase {
            fname: "/tmp/rw_rust_testdata/users.db".to_string(),
        };
        let n = format!("arch_{}", get_current_micro());
        let o = ub.add_user(&n as &str, "episode 8");
        assert_eq!(o.is_ok(), true);
    }

    // make a payment
    // will modify the db file
    {
        let ub = UserBase {
            fname: "/tmp/rw_rust_testdata/users.db".to_string(),
        };
        let o = ub.pay("matt", "dave", 320);
        assert_eq!(o.is_ok(), true);
    }
}

#[test]
fn demo_retrieve_data() {
    {
        let ub = UserBase {
            fname: "/tmp/rw_rust_testdata/users.db".to_string(),
        };
        let o = ub.validate_user("dave", "dave_pw");
        assert!(o.is_ok());
    }
}
