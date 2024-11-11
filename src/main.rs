use std::{collections::HashMap, io::{self, BufRead}};

use html_template_engine::*;



fn main() {
    let mut context = HashMap::new();

    context.insert("name".to_string(), "Bob".to_string());

    context.insert("city".to_string(), "Caracas".to_string());

    for line in io::stdin().lock().lines() {
        match get_content_type(&line.unwrap().clone()) {
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