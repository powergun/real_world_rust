#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::copy;

#[test]
fn demo_copy_file() {
    fn f() -> Result<(), std::io::Error> {
        let mut from_ = File::open("/etc/passwd")?;
        let mut to_ = File::create("/tmp/passwd")?;
        copy(&mut from_, &mut to_)?;
        Ok(())
    }
    f().expect("fail to copy file");
}
