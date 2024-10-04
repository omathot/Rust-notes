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

    // $(,)? only serves to allow trailing , after last item
    ($type:ty, $($arg:expr),+ $(,)?) => {
        <$type>::build($($arg),+)
    };
}

// $($arg:expr),+ has to happen at least once
// $($arg:expr),* can happen zero or many times

macro_rules! make_vec_strings {
    ($($arg:expr),*) => {
        vec![$($arg.to_string()),*]
    };
}

macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    };
    ($x:expr, $($y:expr),+) => {
        $x + add![$($y),+]
    }
}

macro_rules! swap {
    ($x:expr, $y:expr) => {
        let temp = $x;
        $x = $y;
        $y = temp;
    };
}

macro_rules! enum_with_method {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?),*$(,)?
    }) => {
        //
    };
}

macro_rules! make_vec_ints {
    ($($x:expr),*) => {
        vec![$($x), *]
    };
}

// make a struct with implemented methods
macro_rules! make_fancy_struct {
    ($name:ident$(, $($ef:ident),*)?) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub struct $name {
            power: usize,
            $($($ef: String,)*)?
        }
        impl $name {
            pub fn new(n: usize) -> Self {
                $name {
                    power: n,
                    $($($ef: String::new(),)*)?
                }
            }
        }
    };
}
