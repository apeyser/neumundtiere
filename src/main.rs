pub mod term;
pub mod rpn;
pub mod reader;

fn main() {
    term::line::exec();
}
