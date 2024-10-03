#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Foo {
    health: u64,
    dmg: u64,
    name: String,
}
impl Foo {
    pub fn new() -> Self {
        Foo {
            health: 0,
            dmg: 0,
            name: String::new(),
        }
    }
    pub fn build(name: &str) -> Self {
        Foo {
            health: 0,
            dmg: 0,
            name: name.to_string(),
        }
    }
}

// idea: takes any type, builds it with some arg options
macro_rules! make {
    ($type:ty) => {
        <$type>::new()
    };

    ($type:ty, $($arg:expr),+ $(,)?) => {
        <$type>::build($($arg),+)
    };
}
