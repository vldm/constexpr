#[macro_use]
extern crate constexpr;

template!{
    fn get_val<T>(x:T) -> u32 {
        x.val()
    }
}

struct Foo;
impl Foo {
    fn val(&self) -> u32 {
        42
    }
}

struct Baz{x:u32}
impl Baz {
    fn val(&self) -> u32 {
        self.x
    }
}

struct Bar;

fn main() {
    println!("{}", get_val!(Foo));
    println!("{}", get_val!(Baz{x: 12}));
    println!("{}", get_val!(Bar)); // Uncomment tot shoot your leg
}