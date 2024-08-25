use std::env;
// --snip--

/// Just learning about next here
// fn main () {
// 	let mut args = env::args();
// 	println!("{}", args.next().unwrap());
// 	println!("{}", args.next().unwrap());
// 	println!("{}", args.next().unwrap_or("None".to_string()));
// }


// based on io app from rustbook.
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}