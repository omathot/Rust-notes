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
        let test = make![Foo, "hello"];

        assert_eq!(Foo::build("hello"), test);
    }

    #[test]
    fn use_recurrence_macro_fibonacci() {
        let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-2] + a[n - 1]];
        for item in fib.take(10) {
            println!("{}", item);
        }
    }
}
