use std::io;
use std::io::Write;

pub fn echo(args: String) {
    if args.contains(" -n") {
        print!("{}", args.replace(" -n", ""));
        io::stdout().flush().unwrap();
        return;
    }
    println!("{args}");
}
