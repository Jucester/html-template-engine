use std::{collections::HashMap, fs, path::Path, io::{self, BufRead, BufReader}};
use html_template_engine::*;



fn main() {
    let mut context = HashMap::new();

    context.insert("name".to_string(), "Bob".to_string());

    context.insert("city".to_string(), "Caracas".to_string());


    // Reading file:
    let mut html_content = Vec::new();

    let path = Path::new("/home/jucester/Work/Personal/TheOwlymath/Rust/RustPath/Level01/html-template-engine/src/test.html");
    for line in fs::read_to_string(path).unwrap().lines() {
        html_content.push(line.to_string())
    }


    // for line in io::stdin().lock().lines() {
    for line in html_content {
        match get_content_type(&line.clone()) {
            ContentType::TemplateVariable(content) => {
                let html = generate_html(content, context.clone());
                println!("Content");
            }
            ContentType::Literal(text) => println!("Literal"),
            ContentType::Tag(TagType::ForTag) => println!("FOR TAG"),
            ContentType::Tag(TagType::IfTag) => println!("IF TAG"),
            ContentType::Unrecognized => println!("UNRECOGNIZED")
        }
    }
}