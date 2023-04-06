use cursed_ternary::t;

#[allow(clippy::eq_op)]
fn main() {
    let a = t!(0 == 0 ? "True" : "False");
    let b = t!(0 != 0 ? "True" : "False");

    println!("{a:?}");
    println!("{b:?}");
}
