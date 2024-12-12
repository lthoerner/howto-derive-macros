trait Identify {
    fn type_name(&self) -> &'static str;
}

struct FooType;
struct BarType;

impl Identify for FooType {
    fn type_name(&self) -> &'static str {
        "FooType"
    }
}

impl Identify for BarType {
    fn type_name(&self) -> &'static str {
        "BarType"
    }
}

fn main() {
    let my_foo = FooType;
    let my_bar = BarType;

    println!("{}", my_foo.type_name());
    println!("{}", my_bar.type_name());
}
