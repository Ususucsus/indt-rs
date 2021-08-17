use indt::Indent;
use std::io;
use std::io::Write;

#[test]
pub fn indent_test() {
    let stdout = &mut io::stdout();
    let mut indt = Indent::from_writer(stdout);

    indt.more();

    writeln!(indt, "line").unwrap();
}
