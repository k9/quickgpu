pub fn lines(code: Vec<String>, separate: bool) -> String {
    let code = code
        .into_iter()
        .map(|line| line.trim().to_string())
        .filter(|section| !section.is_empty())
        .collect::<Vec<_>>();

    code.join(if separate { "\n\n" } else { "\n" })
}
