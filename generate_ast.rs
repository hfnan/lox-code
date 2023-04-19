use std::{io::{self, Write}, fs};

pub fn generate_ast(output_dir: &str) -> io::Result<()>{

    define_ast(output_dir, "Expr", &[
        "Binary   > left: Box<Expr>, operator: Token, right: Box<Expr>".to_owned(),
        "Grouping > expression: Box<Expr>".to_owned(),
        "Literal  > value: Option<Literal>".to_owned(),
        "Unary    > operator: Token, right: Box<Expr>".to_owned()
    ])?;
    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase()); 
    let mut file = fs::File::create(path)?;

    writeln!(file, "use crate::error::*;")?;   
    writeln!(file, "use crate::token::*;")?;   
    writeln!(file)?;
    
    define_visitor(&mut file, base_name, types)?;
    define_enum(&mut file, base_name, types)?;
    
    for ttype in types {
        let (class_name, fields) = ttype.split_once('>').unwrap();
        let (class_name, fields) = (&format!("{}{}", class_name.trim(), base_name), fields.trim());
        define_type(&mut file, base_name, class_name, fields)?;
    }

    define_impl(&mut file, base_name, types)?;
    Ok(())
}



fn define_fn(file: &mut fs::File, base_name: &str, class_name: &str) ->  io::Result<()> {
    writeln!(file, "    pub fn accept<T>(&self, visitor: &impl {base_name}Visitor<T>) -> Result<T, LoxError> {{")?;
    writeln!(file, "        visitor.visit_{}_expr(self)", class_name.to_lowercase())?;
    writeln!(file, "    }}")?;
    writeln!(file)?;
    Ok(())
}

fn define_impl(file: &mut fs::File, base_name: &str, types: &[String]) -> io::Result<()> {
    writeln!(file, "impl {base_name} {{")?;
    writeln!(file, "    pub fn accept<T>(&self, visitor: &impl {base_name}Visitor<T>) -> Result<T, LoxError> {{")?;
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
        define_fn(file, base_name, class_name)?;
        writeln!(file, "}}")?;
        writeln!(file)?;
    }
    Ok(())
}

fn define_visitor(file: &mut fs::File, base_name: &str, types: &[String]) -> io::Result<()> {
    writeln!(file, "pub trait {base_name}Visitor<T> {{")?;
    for ttype in types {
        let class_name = ttype.split('>').next().unwrap().trim();
        writeln!(file, "   fn visit_{}_expr(&self, expr: &{class_name}{base_name}) -> Result<T, LoxError>;", class_name.to_lowercase())?;
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
