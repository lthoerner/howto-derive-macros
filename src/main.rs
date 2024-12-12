use my_macros::Identify;

trait Identify {
    fn type_name(&self) -> &'static str;
}

#[derive(Identify)]
struct FooType;
#[derive(Identify)]
struct BarType;

fn main() {
    let my_foo = FooType;
    let my_bar = BarType;

    println!("{}", my_foo.type_name());
    println!("{}", my_bar.type_name());
}
