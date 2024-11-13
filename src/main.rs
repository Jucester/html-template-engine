use html_template_engine::*;
use std::{
    collections::HashMap,
    io::BufReader,
    path::Path,
};

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;

fn get_file_content(path: &Path) -> Vec<String> {
    let mut html_content = Vec::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        html_content.push(line.to_string())
    }

    html_content
}

fn main() {
    let mut context = HashMap::new();

    context.insert("name".to_string(), "Bob".to_string());
    context.insert("city".to_string(), "Caracas".to_string());

    // Reading file:
    let path: &Path = Path::new("/home/jucester/Work/Personal/TheOwlymath/Rust/RustPath/Level01/html-template-engine/src/test.html");
    let html_content = get_file_content(path);

    let mut html_content_parsed = Vec::new();

    for line in html_content {
        match get_content_type(&line.clone()) {
            ContentType::TemplateVariable(content) => {
                let html = generate_html(content, context.clone());
                html_content_parsed.push(html);
            }
            ContentType::Literal(text) => {
                html_content_parsed.push(text);
            }
            ContentType::Tag(TagType::ForTag) => println!("FOR TAG"),
            ContentType::Tag(TagType::IfTag) => println!("IF TAG"),
            ContentType::Unrecognized => println!("UNRECOGNIZED"),
        }
    }

    let mut html_bytes: Vec<u8> = Vec::new();
    
    for line in html_content_parsed{
        html_bytes.extend(line.as_bytes());
        html_bytes.push(b'\n');
    }

    let new_file: &Path = Path::new("/home/jucester/Work/Personal/TheOwlymath/Rust/RustPath/Level01/html-template-engine/src/new_file.html");

    let mut f = fs::File::create(new_file).unwrap();

    f.write_all(&html_bytes);
}
