use std::fs::{self, File};
use std::io::{self, Write};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
// Any space source is fine, but we use the width one
enum Class {
    #[display("w-{0}\twidth: {1}px;")]
    Px(String, u8),
    #[display("w-{0}\twidth: {:?}px; /* {1}px */")]
    Rem(String, u8),
    #[display("w-{0}\twidth: {1}%;")]
    Percent(String, f32),
}

pub fn generate() -> io::Result<()> {
    let input_file_path = "./src/generate/generate_space.txt";
    let output_file_path = "./src/classes/space.rs";

    let mut output_file = File::create(output_file_path)?;

    let classes = fs::read_to_string(input_file_path)
        .expect("couldn't open file")
        .lines()
        .map(str::parse::<Class>)
        .filter_map(Result::ok)
        .map(generate_classes)
        .collect::<String>();

    let imports = "\
use bevy::ui::{UiRect, Val};
use el::HasStyle;\
    ";
    let output = format!("{}\n\n{}", imports, classes);
    output_file.write_all(output.as_bytes())
}

fn generate_classes(class: Class) -> String {
    let name = match &class {
        Class::Px(name, _) | Class::Rem(name, _) | Class::Percent(name, _) => {
            name.replace("-", "_").replace("/", "d")
        }
    };

    let value = match &class {
        Class::Px(_, value) | Class::Rem(_, value) => format!("Val::Px({}f32)", value),
        Class::Percent(_, value) => format!("Val::Percent({}f32)", value),
    };

    let mut result = "".to_string();

    result += &generate_size(&name, &value);

    match &class {
        Class::Px(_, _) | Class::Rem(_, _) => {
            result += &generate_margin_and_padding(&name, &value);
        }
        _ => {}
    }

    result
}

fn generate_size(name: &String, value: &String) -> String {
    let sizes = [
        ("w", format!("style.width = {value};")),
        ("h", format!("style.height = {value};")),
        (
            "size",
            format!("style.width = {value};\nstyle.height = {value};"),
        ),
    ];

    let mut result = "".to_string();

    for (prefix, setter) in sizes.iter() {
        result += &format!(
            r#"
/// ```
///
/// {value}
/// ```
pub fn {prefix}_{name}(bundle: &mut impl HasStyle) {{
    let style = bundle.style();
    {setter}
}}
"#
        );
    }

    result
}

fn generate_margin_and_padding(name: &String, value: &String) -> String {
    let paddings = [
        ("p", format!("style.padding = UiRect::all({value});")),
        ("pt", format!("style.padding.top = {value};")),
        ("pb", format!("style.padding.bottom = {value};")),
        ("pl", format!("style.padding.left = {value};")),
        ("pr", format!("style.padding.right = {value};")),
        (
            "px",
            format!(
                "
style.padding.top = {value};\n
style.padding.bottom = {value};"
            ),
        ),
        (
            "py",
            format!(
                "
        style.padding.top = {value};\n
        style.padding.bottom = {value};"
            ),
        ),
        ("m", format!("style.margin = UiRect::all({value});")),
        ("mt", format!("style.margin.top = {value};")),
        ("mb", format!("style.margin.bottom = {value};")),
        ("ml", format!("style.margin.left = {value};")),
        ("mr", format!("style.margin.right = {value};")),
        (
            "mx",
            format!(
                "
style.margin.top = {value};\n
style.margin.bottom = {value};"
            ),
        ),
        (
            "my",
            format!(
                "
        style.margin.top = {value};\n
        style.margin.bottom = {value};"
            ),
        ),
    ];
    let mut result = "".to_string();
    for (prefix, setter) in paddings.iter() {
        result += &format!(
            r#"
/// ```
///
/// {value}
/// ```
pub fn {prefix}_{name}(bundle: &mut impl HasStyle) {{
    let style = bundle.style();
    {setter}
}}
"#
        );
    }

    result
}
