use parse_display::{Display, FromStr};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Display, FromStr, PartialEq, Debug)]
// Any color source is fine, but we use the background-color one
#[display("bg-{name}\tbackground-color: rgb({color.0} {color.1} {color.2});")]
struct Class {
    pub name: String,
    #[from_str(default)]
    color: (u8, u8, u8),
}

pub fn generate() -> io::Result<()> {
    let input_file_path = "./src/generate/generate_colors.txt";
    let output_file_path = "./src/classes/colors.rs";

    let mut output_file = File::create(output_file_path)?;

    let classes = fs::read_to_string(input_file_path)
        .expect("couldn't open file")
        .lines()
        .map(str::parse::<Class>)
        .filter_map(Result::ok)
        .map(generate_classes)
        .collect::<String>();

    let imports = "
use bevy::render::color::Color;
use el::{HasBackgroundColor, HasText};
    ";
    let output = format!("{}\n\n{}", imports, classes);
    output_file.write_all(output.as_bytes())
}

fn generate_classes(class: Class) -> String {
    let name = class.name.replace("-", "_");

    let r: u8 = class.color.0;
    let g: u8 = class.color.1;
    let b: u8 = class.color.2;

    let value = format!("Color::rgb_u8({r}, {g}, {b})");

    let background_color = generate_background_color(&name, &value);
    let text_color = generate_text_color(&name, &value);

    format!("{}\n{}", background_color, text_color)
}

fn generate_background_color(name: &String, value: &String) -> String {
    format!(
        r#"/// ```
///
/// {value}
/// ```
pub fn bg_{name}(bundle: &mut impl HasBackgroundColor) {{
    let background_color = bundle.background_color();
    background_color.0 = {value};
}}
"#
    )
}

fn generate_text_color(name: &String, value: &String) -> String {
    format!(
        r#"/// ```
///
/// {value}
/// ```
pub fn text_{name}(bundle: &mut impl HasText) {{
    let mut text = bundle.text();
    text.sections.iter_mut().for_each(|section| {{
        section.style.color = {value};
    }});
}}
"#
    )
}
