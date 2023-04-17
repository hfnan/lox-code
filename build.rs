use generate_ast::generate_ast;
mod generate_ast;

fn main() {
    generate_ast("src/").unwrap();
}
