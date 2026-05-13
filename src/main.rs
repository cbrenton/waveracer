mod helpers;
mod math;

fn main() {
    dbg!(helpers::random_double());
    let foo = math::DInterval::EMPTY;
    dbg!(foo);
}
