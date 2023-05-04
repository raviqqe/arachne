#[derive(Debug, Default)]
enum Expression {
    #[default]
    None,
    Symbol(String),
}

fn main() {
    println!("Hello, world!");
}

fn parse_expression() -> Expression {
    todo!()
}
