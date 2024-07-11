use repl::Repl;
fn main() {
    let  new_repl = Repl::new(String::from(">> "));
    new_repl.start();
}
    