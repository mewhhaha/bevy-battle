mod generate_colors;
mod generate_space;

fn main() {
    let result = generate_colors::generate();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    let result = generate_space::generate();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
