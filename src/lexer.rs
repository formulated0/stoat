use std::fs::read_to_string;

pub fn lex_file(file_path: &str) -> Vec<(String, usize, usize, String)> {
    let content = read_to_string(file_path).unwrap();
    let mut result = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        for (col, tok) in lex_line(line) {
            result.push((file_path.to_string(), line_idx + 1, col, tok.to_string())); // TODO: remove the +1 for 0-based line
        }
    }

    result
}

pub fn lex_line(line: &str) -> Vec<(usize, &str)> {
    let mut i = 0;
    let bytes = line.as_bytes();
    let len = line.len();
    let mut tokens = Vec::new();

    while i < len {
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        if i >= len {
            break;
        }

        let start = i;

        while i < len && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        let token = &line[start..i];
        let column = start + 1; // TODO: remove the +1 for 0-based column

        tokens.push((column, token))
    }
    tokens
}
