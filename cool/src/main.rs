


fn testing_symbol() {
    println!("Insane");
}
fn main() {
    let i = 52;
    let c = 5;
    let mut cs = cool_sys::CoolStruct {
        x: 0,
        y: 0
    };
    unsafe { cool_sys::cool_function(i, c, &mut cs) };
    unsafe { cool_sys::test() };
    testing_symbol();
    println!("Hello, world!");
}
