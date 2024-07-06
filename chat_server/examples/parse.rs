use std::str::FromStr;

use regex::Regex;

trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: Default + FromStr,
{
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        // let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

trait Draw {
    fn draw(&self);
}

#[derive(Copy, Clone, Debug)]
struct SelectBox;
impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox");
    }
}

struct Button;
impl Draw for Button {
    fn draw(&self) {
        println!("Button");
    }
}

fn main() {
    assert_eq!(u32::parse("123ddd").unwrap(), 123);
    assert_eq!(u8::parse("255 hhh www").expect(""), 255);

    let i1: i8 = Default::default();
    let i2 = i8::default();
    assert_eq!(i1, i2);

    let c = || {
        println!("c called");
        Default::default()
    };
    let c2: u8 = c();
    assert_eq!(c2, 0);

    let sb = SelectBox;
    let _v = [sb];

    let mut vv = Vec::<Box<dyn Draw>>::new();
    vv.push(Box::new(sb));

    let b = Button;
    vv.push(Box::new(b));

    for e in vv.iter() {
        e.draw();
    }

    // let dr: &dyn Draw = &Button;

    // let v2 = Vec::<u8>::new();
}
