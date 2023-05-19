use std::{io::{self, Write}, fs};

pub fn generate_ast(output_dir: &str) -> io::Result<()>{

    define_ast(output_dir, "Expr", &[
        "Assign   > name: Token, value: Box<Expr>".to_owned(),
        "Binary   > left: Box<Expr>, operator: Token, right: Box<Expr>".to_owned(),
        "Call     > callee: Box<Expr>, paren: Token, arguments: Vec<Expr>".to_owned(),
        "Grouping > expression: Box<Expr>".to_owned(),
        "Literal  > value: Option<Object>".to_owned(),
        "Logical  > left: Box<Expr>, operator: Token, right: Box<Expr>".to_owned(),
        "Unary    > operator: Token, right: Box<Expr>".to_owned(),
        "Variable > name: Token".to_owned(),
    ])?;

    define_ast(output_dir, "Stmt", &[
        "Break      > line: usize".to_owned(),
        "Block      > statements: Vec<Stmt>".to_owned(),
        "Expression > expression: Expr".to_owned(),
        "Function   > name: Token, parameters: Rc<Vec<Token>>, body: Rc<Vec<Stmt>>".to_owned(),
        "If         > condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>".to_owned(),
        "Print      > expression: Expr".to_owned(),
        "Return     > keyword: Token, value: Expr".to_owned(),
        "Var        > name: Token, initializer: Option<Expr>".to_owned(),
        "While      > condition: Expr, body: Box<Stmt>".to_owned(),
    ])?;
    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase()); 
    let mut file = fs::File::create(path)?;

    writeln!(file, "use crate::error::*;")?;   
    writeln!(file, "use crate::token::*;")?;   
    writeln!(file, "use std::rc::Rc;")?;
    writeln!(file, "use std::hash::Hash;")?;
    if let "Expr" = base_name {
        writeln!(file, "use crate::object::*;")?;
    }
    if let "Stmt" = base_name {
        writeln!(file, "use crate::expr::*;")?;
    }  
    writeln!(file)?;
    
    define_visitor(&mut file, base_name, types, "Output")?;
    define_enum(&mut file, base_name, types)?;
    
    for ttype in types {
        let (class_name, fields) = ttype.split_once('>').unwrap();
        let (class_name, fields) = (&format!("{}{}", class_name.trim(), base_name), fields.trim());
        define_type(&mut file, class_name, fields)?;
    }

    define_impl(&mut file, base_name, types, "Output")?;
    Ok(())
}

fn define_fn(file: &mut fs::File, base_name: &str, class_name: &str, assotype: &str) ->  io::Result<()> {
    writeln!(file, "    pub fn accept<U>(self: &Rc<{class_name}{base_name}>, visitor: &mut impl {base_name}Visitor<{assotype} = U>) -> Result<U, LoxError> {{")?;
    writeln!(file, "        visitor.visit_{}_{}(Rc::clone(self))", class_name.to_lowercase(), base_name.to_lowercase())?;
    writeln!(file, "    }}")?;
    writeln!(file)?;
    Ok(())
}

fn define_impl(file: &mut fs::File, base_name: &str, types: &[String], assotype: &str) -> io::Result<()> {
    writeln!(file, "impl {base_name} {{")?;
    writeln!(file, "    pub fn accept<U>(&self, visitor: &mut impl {base_name}Visitor<{assotype} = U>) -> Result<U, LoxError> {{")?;
    writeln!(file, "        match self {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "            {base_name}::{class_name}({}stmt) => {}stmt.accept(visitor),", class_name.to_lowercase(), class_name.to_lowercase())?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl PartialEq for {base_name} {{")?;
    writeln!(file, "    fn eq(&self, other: &Self) -> bool {{")?;
    writeln!(file, "        match (self, other) {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "            ({base_name}::{class_name}(a), {base_name}::{class_name}(b)) => Rc::ptr_eq(a, b),")?;
    }
    writeln!(file, "            _ => false,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl Eq for {base_name} {{}}")?;
    writeln!(file)?;

    writeln!(file, "impl Hash for {base_name} {{")?; 
    writeln!(file, "    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {{")?;
    writeln!(file, "        match self {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "            {base_name}::{class_name}(a) => hasher.write_usize(Rc::as_ptr(a) as usize),")?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "impl {class_name}{base_name} {{")?;
        define_fn(file, base_name, class_name, assotype)?;
        writeln!(file, "}}")?;
        writeln!(file)?;
    }
    Ok(())
}

fn define_visitor(file: &mut fs::File, base_name: &str, types: &[String], assotype: &str) -> io::Result<()> {
    writeln!(file, "pub trait {base_name}Visitor {{")?;
    writeln!(file, "    type {assotype};")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "    fn visit_{}_{}(&mut self, {}: Rc<{class_name}{base_name}>) -> Result<Self::{assotype}, LoxError>;", class_name.to_lowercase(), base_name.to_lowercase(), base_name.to_lowercase())?;
    }   
    writeln!(file, "}}")?;
    writeln!(file)
}

fn define_enum(file: &mut fs::File, base_name: &str, types: &[String]) -> io::Result<()>{
    writeln!(file, "pub enum {base_name} {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "    {class_name}(Rc<{class_name}{base_name}>),")?;   
    }
    writeln!(file, "}}")?;
    writeln!(file)
}

fn define_type(file: &mut fs::File, class_name: &str, fields: &str) -> io::Result<()> {
    writeln!(file, "pub struct {class_name} {{")?;
    for field in fields.split(',') {
        writeln!(file, "    pub {},", field.trim())?;
    }
    writeln!(file, "}}")?;
    writeln!(file)
}
