// 括弧をインデントして見やすくする
pub fn indent(s: String) -> String {
    let indent = "    ";
    s.replace("{}", "@")
        .chars()
        .fold(
            (String::new(), 0),
            |(acc, width): (String, usize), c: char| match c {
                '{' => (
                    format!("{} {}\n{}", acc, c, indent.repeat(width + 1)),
                    width + 1,
                ),
                '}' => (
                    format!("{}\n{}{}", acc, indent.repeat(width - 1), c),
                    width - 1,
                ),
                ';' => (format!("{} {}\n{}", acc, c, indent.repeat(width)), width),
                _ => (format!("{}{}", acc, c), width),
            },
        )
        .0
        .replace("@", " {}")
}
