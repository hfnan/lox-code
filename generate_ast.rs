use std::{io::{self, Write}, fs};

pub fn generate_ast(output_dir: &str) -> io::Result<()>{

    define_ast(output_dir, "Expr", &[
        "Binary   > left: Box<Expr>, operator: Token, right: Box<Expr>".to_owned(),
        "Grouping > expression: Box<Expr>".to_owned(),
        "Literal  > value: Option<Object>".to_owned(),
        "Unary    > operator: Token, right: Box<Expr>".to_owned(),
        "Variable > name: Token".to_owned(),
    ])?;

    define_ast(output_dir, "Stmt", &[
        "Expression > expression: Box<Expr>".to_owned(),
        "Print      > expression: Box<Expr>".to_owned(),
        "Var        > name: Token, initializer: Option<Expr>".to_owned(),
    ])?;
    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase()); 
    let mut file = fs::File::create(path)?;

    writeln!(file, "use crate::error::*;")?;   
    writeln!(file, "use crate::token::*;")?;   
    writeln!(file, "use crate::object::*;")?; 
    if let "Stmt" = base_name {
        writeln!(file, "use crate::expr::*;")?;
    }  
    writeln!(file)?;
    
    define_visitor(&mut file, base_name, types, "Output")?;
    define_enum(&mut file, base_name, types)?;
    
    for ttype in types {
        let (class_name, fields) = ttype.split_once('>').unwrap();
        let (class_name, fields) = (&format!("{}{}", class_name.trim(), base_name), fields.trim());
        define_type(&mut file, base_name, class_name, fields)?;
    }

    define_impl(&mut file, base_name, types, "Output")?;
    Ok(())
}



fn define_fn(file: &mut fs::File, base_name: &str, class_name: &str, assotype: &str) ->  io::Result<()> {
    writeln!(file, "    pub fn accept<U>(&self, visitor: &impl {base_name}Visitor<{assotype} = U>) -> Result<U, LoxError> {{")?;
    writeln!(file, "        visitor.visit_{}_{}(self)", class_name.to_lowercase(), base_name.to_lowercase())?;
    writeln!(file, "    }}")?;
    writeln!(file)?;
    Ok(())
}

fn define_impl(file: &mut fs::File, base_name: &str, types: &[String], assotype: &str) -> io::Result<()> {
    writeln!(file, "impl {base_name} {{")?;
    writeln!(file, "    pub fn accept<U>(&self, visitor: &impl {base_name}Visitor<{assotype} = U>) -> Result<U, LoxError> {{")?;
    writeln!(file, "        match self {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "            {base_name}::{class_name}({}) => {}.accept(visitor),", class_name.to_lowercase(), class_name.to_lowercase())?;
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
        writeln!(file, "    fn visit_{}_{}(&self, {}: &{class_name}{base_name}) -> Result<Self::{assotype}, LoxError>;", class_name.to_lowercase(), base_name.to_lowercase(), base_name.to_lowercase())?;
    }   
    writeln!(file, "}}")?;
    writeln!(file)
}

fn define_enum(file: &mut fs::File, base_name: &str, types: &[String]) -> io::Result<()>{
    writeln!(file, "pub enum {base_name} {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "    {class_name}({class_name}{base_name}),")?;   
    }
    writeln!(file, "}}")?;
    writeln!(file)
}

fn define_type(file: &mut fs::File, base_name: &str, class_name: &str, fields: &str) -> io::Result<()> {
    writeln!(file, "pub struct {class_name} {{")?;
    for field in fields.split(',') {
        writeln!(file, "    pub {},", field.trim())?;
    }
    writeln!(file, "}}")?;
    writeln!(file)
}
