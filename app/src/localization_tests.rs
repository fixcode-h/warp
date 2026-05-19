use super::{localize, zh_cn};

#[test]
fn zh_cn_translates_known_settings_labels() {
    assert_eq!(zh_cn("Account"), Some("账户"));
    assert_eq!(zh_cn("Keyboard shortcuts"), Some("键盘快捷键"));
    assert_eq!(zh_cn("Warp Agent"), Some("Warp 智能体"));
}

#[test]
fn localize_falls_back_to_source_text() {
    assert_eq!(localize("Untranslated label"), "Untranslated label");
}
