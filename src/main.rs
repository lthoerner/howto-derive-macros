use my_macros::Identify;

trait Identify {
    fn type_name(&self) -> &'static str;
    fn friendly_type_name(&self) -> &'static str;
}

#[derive(Identify)]
struct FooType;
#[derive(Identify)]
struct BarType;

fn main() {
    let my_foo = FooType;
    let my_bar = BarType;

    println!("{}", my_foo.type_name());
    println!("{} :)", my_foo.friendly_type_name());
    println!("{}", my_bar.type_name());
    println!("{} :)", my_bar.friendly_type_name());
}
