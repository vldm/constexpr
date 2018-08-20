#[macro_use]
extern crate templates;

template!{
    fn print_some<T>(var:T) -> String {
        format!("{}", var)
    }
}

#[test]
fn assert_display() {
    let x = format!("{}", 12);
    assert_eq!(print_some!(12), x)
}