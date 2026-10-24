fn is_zh(locale: &str) -> bool {
    locale == "zh" || locale == "zh-CN" || locale == "cn"
}

fn is_ru(locale: &str) -> bool {
    locale == "ru" || locale.starts_with("ru-")
}

pub fn quit(locale: &str) -> &'static str {
    if is_zh(locale) {
        "退出 UsageBar"
    } else if is_ru(locale) {
        "Выйти из UsageBar"
    } else {
        "Quit UsageBar"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit_supports_russian_and_falls_back_to_english() {
        assert_eq!(quit("ru"), "Выйти из UsageBar");
        assert_eq!(quit("ru-RU"), "Выйти из UsageBar");
        assert_eq!(quit("unsupported"), "Quit UsageBar");
    }
}
