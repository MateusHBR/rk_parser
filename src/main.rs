use std::env;
use std::io::Write;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const ERROR_MESSAGE: &str = "FALHA AO CONVERTER LINHA";
const INPUT_FILE: &str = "input.txt";
const OUTPUT_FILE: &str = "output.txt";

fn main() {
    let current_dir = env::current_dir()
        .expect("Falha ao encontrar o diretório atual")
        .display()
        .to_string();

    let file: File = File::open(format!("{current_dir}/{INPUT_FILE}"))
        .expect("Falha ao encontrar o arquivo especificado");

    let reader = BufReader::new(&file);

    let mut result_file =
        File::create(format!("{current_dir}/{OUTPUT_FILE}")).expect("Falha ao criar o arquivo");

    reader
        .lines()
        .map(|line| line.unwrap_or(ERROR_MESSAGE.to_string()))
        .map(format)
        .for_each(|line| write_in_file(line, &mut result_file));
}

fn write_in_file(line: String, file: &mut File) {
    file.write_all(format!("{}\r", line).as_bytes())
        .expect("Falha ao escrever no arquivo");
}

fn format(line: String) -> String {
    if line == ERROR_MESSAGE {
        return line;
    }

    let splitted: Vec<String> = line
        .trim()
        .split(" ")
        .map(|v| v.trim().to_string())
        .collect();
    let splitted = splitted.as_slice();

    if let [code, tail @ ..] = splitted {
        return format!("{:0>13}   {}", code, tail.join(" ").trim());
    }

    return line.to_string();
}
