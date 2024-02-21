use super::super::get_example_locale;
use crate::utils::{get_locale_string, LocaleCategory};

#[test]
fn parsing_1() {
    let locales = get_example_locale();

    let mut left = get_locale_string(
        &locales,
        LocaleCategory::Parsing,
        "Failed to parse the document. [Line: {LINE_NUMBER}]",
    );
    left = left.replace("{LINE_NUMBER}", 1.to_string().as_str());
    let right = String::from("문서를 읽지 못했습니다. [줄: 1]");

    assert_eq!(left, right);
}

#[test]
fn parsing_2() {
    let locales = get_example_locale();

    let left = get_locale_string(
        &locales,
        LocaleCategory::Parsing,
        "Failed to parse the document.",
    );
    let right = String::from("문서를 읽지 못했습니다.");

    assert_eq!(left, right);
}

#[test]
fn formatting_1() {
    let locales = get_example_locale();

    let left = get_locale_string(
        &locales,
        LocaleCategory::Formatting,
        "Failed to read options. Please make sure there is no option with an empty value.",
    );
    let right =
        String::from("옵션을 읽지 못했습니다. 값이 비어있는 옵션이 없는지 다시 확인해주세요.");

    assert_eq!(left, right);
}

#[test]
fn formatting_2() {
    let locales = get_example_locale();

    let left = get_locale_string(
        &locales,
        LocaleCategory::Formatting,
        "Failed to read option properties.",
    );
    let right = String::from("옵션 프로퍼티를 읽지 못했습니다.");

    assert_eq!(left, right);
}
