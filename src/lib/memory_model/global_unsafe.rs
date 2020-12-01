#[allow(dead_code)]
static mut A: i32 = 0;

#[test]
fn demo_use_global_var() {
    unsafe {
        A = 1;
    }

}