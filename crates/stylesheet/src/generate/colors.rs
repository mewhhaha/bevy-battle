use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn generate() -> io::Result<()> {
    let input_file_path = "src/generate/colors.txt";
    let output_file_path = "src/stylesheet/colors.rs";

    let mut output_file = File::create(output_file_path)?;

    if let Ok(lines) = read_lines(input_file_path) {
        for line in lines {
            if let Ok(css_line) = line {
                if let Some((class, color)) = parse_css_line(&css_line) {
                    let rust_function = generate_rust_function(&class, &color);
                    writeln!(output_file, "{}", rust_function)?;
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_css_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        let class = parts[0].to_string();
        let color = parts[1].to_string();
        Some((class, color))
    } else {
        None
    }
}

fn generate_rust_function(class: &str, color: &str) -> String {
    let rust_function_name = class.replace("-", "_");
    let rgb_values = color.replace("rgb(", "").replace(")", "");
    let rgb_parts: Vec<&str> = rgb_values.split_whitespace().collect();
    let r: u8 = rgb_parts[0].parse().unwrap();
    let g: u8 = rgb_parts[1].parse().unwrap();
    let b: u8 = rgb_parts[2].parse().unwrap();

    format!(
        r#"/// ```
///
/// Color::rgb_u8({}, {}, {})
/// ```
pub fn {}(bundle: &mut impl HasBackgroundColor) {{
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8({}, {}, {});
}}
"#,
        r, g, b, rust_function_name, r, g, b
    )
}
