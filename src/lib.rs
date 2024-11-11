pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Notes on #derive
// #[derive] nos permite usar implementaciones por defecto existentes en algunas partes que necesitemos
// PartialEq nos permite comparar los content types. Y debug nos imprime los valores en la consola.

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}


#[derive(PartialEq, Debug)]
pub enum ContentType {
    Literal (string),
    TemplateVariable (ExpressionData),
    Tag (TagType),
    Unrecognized,
}

#[cfg(test)]
mod tests {
    use super::*;
}

#[test]
fn check_literal_test() {
    let s = "<h1>Hello world </h1>";
    assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s))
}

#[test]
fn check_template_var_test() {
    let content = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: "name".to_string(),
        tail: Some(" ,welcome".to_string()),
    };

    assert_eq!(
        ContentType::TemplateVariable(content),
        get_content_type("Hi {{name}} ,welcome")
    );
}

#[test]
fn check_for_tag_test() {
    assert_eq!(
        ContentType::Tag(TagType::ForTag),
        get_content_type("{% for name in names %} ,welcome")
    );
}


#[test]
fn check_if_tag_test() {
    assert_eq!(
        ContentType::Tag(TagType::IfTag),
        get_content_type("{% if name === 'Bob' %}")
    );
}


