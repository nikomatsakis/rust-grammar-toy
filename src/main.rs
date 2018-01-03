use std::env;

mod grammar;

#[test]
fn ok() {
    let pairs = [
        ("foo", "foo"),
        ("foo<bar,>", "foo<bar>"),
        ("foo<'a, bar,>", "foo<'a, bar>"),
        ("Box<Iterator + Foo>", "Box<(Iterator + Foo)>"),
        ("&(Iterator + Foo)", "&((Iterator + Foo))"),
        ("Box<'a + Iterator>", "Box<('a + Iterator)>"),
        ("Box<impl 'a + Iterator>", "Box<impl ('a + Iterator)>"),
        ("Box<dyn 'a + Iterator>", "Box<dyn ('a + Iterator)>"),
        ("Box<dyn ('a + Iterator) + Send>", "Box<dyn ((('a + Iterator)) + Send)>"),
    ];

    for &(input, output) in &pairs {
        match grammar::parse_Ty(input) {
            Ok(r) => assert_eq!(output, r, "input `{}` parsed funny", input),
            Err(e) => panic!("input `{}` did not parse: {:?}", input, e),
        }
    }

    let errors = [
        "&Iterator + Foo"
    ];

    for &input in &errors {
        match grammar::parse_Ty(input) {
            Ok(r) => panic!("input `{}` successfully parsed as `{}`", input, r),
            Err(_) => (),
        }
    }
}

fn main() {
    for arg in env::args().skip(1) {
        println!("Parsing `{}`...", arg);
        match grammar::parse_Ty(&arg) {
            Ok(r) => println!("parsed to: {}", r),
            Err(e) => println!("error: {:?}", e),
        }
    }
}

fn join(args: Vec<String>) -> String {
    let mut buffer = String::new();
    for (index, arg) in args.iter().enumerate() {
        if index > 0 {
            buffer.push_str(", ");
        }
        buffer.push_str(arg);
    }
    buffer
}
