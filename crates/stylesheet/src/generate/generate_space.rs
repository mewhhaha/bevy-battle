use std::fs::{self, File};
use std::io::{self, Write};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
// Any space source is fine, but we use the width one
enum Class {
    #[display("w-{0}\twidth: {1}px;")]
    Px(String, u32),
    #[display("w-{0}\twidth: {2}rem; /* {1}px */")]
    Rem(String, u32, f32),
    #[display("w-{0}\twidth: {1}%;")]
    Percent(String, f32),
    #[display("w-auto\twidth: auto;")]
    Auto,
}

pub fn generate() -> io::Result<()> {
    let input_file_path = "./src/generate/generate_space.txt";
    let output_file_path = "./src/classes/space.rs";

    let mut output_file = File::create(output_file_path)?;

    let classes = fs::read_to_string(input_file_path)
        .expect("couldn't open file")
        .lines()
        .map(str::parse::<Class>)
        .map(|res| res.expect("Parse failed"))
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
        Class::Px(name, _) | Class::Rem(name, _, _) | Class::Percent(name, _) => {
            name.replace("-", "_").replace("/", "d").replace(".", "p")
        }
        Class::Auto => "auto".to_string(),
    };

    let value = match &class {
        Class::Px(_, value) | Class::Rem(_, value, _) => format!("Val::Px({}f32)", value),
        Class::Percent(_, value) => format!("Val::Percent({}f32)", value),
        Class::Auto => "Val::Auto".to_string(),
    };

    let mut result = "".to_string();

    result += &generate_percent_and_px(&name, &value);

    match &class {
        Class::Px(_, _) | Class::Rem(_, _, _) | Class::Auto => {
            result += &generate_px(&name, &value);
        }
        _ => {}
    }

    result
}

fn generate_percent_and_px(name: &String, value: &String) -> String {
    let sizes = [
        ("w", format!("style.width = {value};")),
        ("h", format!("style.height = {value};")),
        (
            "size",
            format!("style.width = {value};\nstyle.height = {value};"),
        ),
        ("basis", format!("style.flex_basis = {value};")),
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

fn generate_px(name: &String, value: &String) -> String {
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
style.padding.top = {value};
style.padding.bottom = {value};"
            ),
        ),
        (
            "py",
            format!(
                "
        style.padding.top = {value};
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
style.margin.top = {value};
style.margin.bottom = {value};"
            ),
        ),
        (
            "my",
            format!(
                "
style.margin.top = {value};
style.margin.bottom = {value};"
            ),
        ),
        ("gap_y", format!("style.column_gap = {value};")),
        ("gap_x", format!("style.row_gap = {value};")),
        (
            "gap",
            format!(
                "
style.column_gap = {value};
style.row_gap = {value};"
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
