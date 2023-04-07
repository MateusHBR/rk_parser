fn main() {
    let input = "
    123   Remédio
    12356      Luva
    12  Mochila
    1 Bolsa
    3 Bolsa de couro
    "
    .to_string();

    let lines: Vec<String> = input.lines().map(format).collect();
    lines.iter().for_each(|v| println!("{}", v));
}

fn format(line: &str) -> String {
    let splitted: Vec<String> = line
        .trim()
        .split(" ")
        .map(|v| v.trim().to_string())
        .collect();

    let splitted = splitted.as_slice();

    if let [code, tail @ ..] = splitted {
        return format!("{:0>13}   {}", code, tail.join(" ").trim());
    } else {
        return line.to_string();
    }
}
