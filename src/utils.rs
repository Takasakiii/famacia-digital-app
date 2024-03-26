pub fn get_inline_style(css: &[(&str, Option<&str>)]) -> String {
    css.iter()
        .filter(|(_, v)| v.is_some())
        .map(|(k, v)| format!("{}: {}", k, v.unwrap()))
        .collect::<Vec<_>>()
        .join(";")
}
