use std::env;

const INPUT_FILE: &str = "input.txt";
const OUTPUT_FILE: &str = "output.txt";

fn main() {
    let current_dir = env::current_dir()
        .expect("Falha ao encontrar o diretório atual")
        .display()
        .to_string();

    rk_parser::api::format_file(
        format!("{current_dir}/{INPUT_FILE}"),
        format!("{current_dir}/{OUTPUT_FILE}"),
    );
}
