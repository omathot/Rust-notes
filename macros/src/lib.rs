#[macro_use]
mod recurrence;

#[macro_use]
mod make;

#[cfg(test)]
mod tests {

    use super::*;
    use make::Foo;

    #[test]
    fn use_make_foo_macro() {
        let test = make![Foo];
        let s = make![String];
        let n = make![Vec<String>];

        let v: Vec<String> = Vec::new();
        assert_eq!(v, n);
    }

    #[test]
    fn make_variating_name_struct() {
        let test = make![Foo, "hello",];

        assert_eq!(Foo::build("hello"), test);
    }

    #[test]
    fn make_vec_of_strings() {
        let v = make_vec_strings!["hey", "bye", "help"];

        assert_eq!(vec!["hey", "bye", "help"], v);
    }

    #[test]
    fn swap_macro() {
        let mut s1 = String::from("Hey");
        let mut s2 = String::from("Bye");

        swap![s1, s2]; //warning for manual swap, nothing wrong
    }

    #[test]
    fn add_macro() {
        let x = add![1, 1];
        let y = add![1, 1, 1, 1, 1];

        assert_eq!(2, x);
        assert_eq!(5, y);
    }

    #[test]
    fn make_vec_ints_macro() {
        let v = make_vec_ints![1, 2, 3];

        assert_eq!(vec![1, 2, 3], v);
    }

    make_fancy_struct!(Test);
    make_fancy_struct!(TestWithNames, name, ability, weakness);
    #[test]
    fn make_fancy_struct_macro() {
        let s = Test { power: 1 };
        let s2 = Test::new(2);

        let ss = TestWithNames {
            power: 1,
            name: String::from("John"),
            ability: String::from("speed"),
            weakness: String::from("cold"),
        };
        let ss2 = TestWithNames::new(1);

        assert_eq!(Test { power: 1 }, s);
        assert_eq!(Test { power: 2 }, s2);
        assert_eq!("John".to_string(), ss.name);
        assert_eq!("", ss2.name);
    }

    #[test]
    fn use_recurrence_macro_fibonacci() {
        let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-2] + a[n - 1]];
        for item in fib.take(10) {
            println!("{}", item);
        }
    }
}
