pub fn human_format_bytes(bytes: u64) -> String {
    let base: u64 = 1024;

    if bytes < base {
        return format!("{} B", bytes);
    }

    let prefixes = ["K", "M", "G", "T", "P", "E"];
    let exponent = (bytes as f64).log(base as f64) as u32;

    format!(
        "{:.1} {}iB",
        (bytes as f64) / (base.pow(exponent) as f64),
        prefixes[(exponent - 1) as usize]
    )
}

pub fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&apos;"),
        _ => None,
    }
}
