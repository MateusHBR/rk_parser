use std::io::Write;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const ERROR_MESSAGE: &str = "FALHA_AO_CONVERTER_LINHA";
const COLUMNS: usize = 13;

pub fn format_file(input_file: String, output_file: String) {
    let input_file: File =
        File::open(input_file).expect("Falha ao encontrar o arquivo especificado");

    let input_reader = BufReader::new(&input_file);

    let mut output_file = File::create(output_file).expect("Falha ao criar o arquivo");

    input_reader
        .lines()
        .map(|line| line.unwrap_or(ERROR_MESSAGE.to_string()))
        .map(format)
        .for_each(|line| write_line(line, &mut output_file));
}

fn write_line(line: String, file: &mut File) {
    file.write(format!("{}\r", line).as_bytes())
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
        return format!("{:0>COLUMNS$}   {}", code, tail.join(" ").trim());
    }

    return line.to_string();
}
