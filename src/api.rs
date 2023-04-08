use std::io::Write;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const ERROR_MESSAGE: &str = "FALHA_AO_CONVERTER_LINHA";

pub fn format_file(input_file: String, output_file: String, number_of_columns: usize) {
    println!("Formatando arquivo: {}", &input_file);
    println!("Arquivo resultado: {}", &output_file);

    let input_file: File =
        File::open(input_file).expect("Falha ao encontrar o arquivo especificado");

    let input_reader = BufReader::new(&input_file);

    let mut output_file =
        File::create(&output_file).expect(&format!("Falha ao criar o arquivo {}", &output_file));

    input_reader
        .lines()
        .map(|line| line.unwrap_or(ERROR_MESSAGE.to_string()))
        .map(|line| format(line, &number_of_columns))
        .for_each(|line| write_line(line, &mut output_file));
}

fn write_line(line: String, file: &mut File) {
    file.write(format!("{}\r", line).as_bytes())
        .expect("Falha ao escrever no arquivo");
}

fn format(line: String, number_of_columns: &usize) -> String {
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
        return format!("{:0>number_of_columns$}   {}", code, tail.join(" ").trim());
    }

    return line.to_string();
}
