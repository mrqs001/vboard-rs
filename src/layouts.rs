#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyBackend {
    Text,
    Special(&'static str),
    Modifier(&'static str),
}

#[derive(Clone, Copy, Debug)]
pub struct KeyDef {
    pub id: &'static str,
    pub width: i32,
    pub normal: &'static str,
    pub shifted: Option<&'static str>,
    pub backend: KeyBackend,
}

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub code: &'static str,
    pub name: &'static str,
    pub xkb_layout: &'static str,
    pub rows: &'static [&'static [KeyDef]],
}

impl KeyDef {
    pub fn label(self, shifted: bool) -> &'static str {
        if shifted {
            self.shifted.unwrap_or(self.normal)
        } else {
            self.normal
        }
    }

    pub fn text_output(self, shifted: bool) -> &'static str {
        if shifted {
            self.shifted.unwrap_or(self.normal)
        } else {
            self.normal
        }
    }
}

const US_ROW_1: &[KeyDef] = &[
    KeyDef {
        id: "esc",
        width: 2,
        normal: "Esc",
        shifted: None,
        backend: KeyBackend::Special("escape"),
    },
    KeyDef {
        id: "1",
        width: 2,
        normal: "1",
        shifted: Some("!"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "2",
        width: 2,
        normal: "2",
        shifted: Some("@"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "3",
        width: 2,
        normal: "3",
        shifted: Some("#"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "4",
        width: 2,
        normal: "4",
        shifted: Some("$"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "5",
        width: 2,
        normal: "5",
        shifted: Some("%"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "6",
        width: 2,
        normal: "6",
        shifted: Some("^"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "7",
        width: 2,
        normal: "7",
        shifted: Some("&"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "8",
        width: 2,
        normal: "8",
        shifted: Some("*"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "9",
        width: 2,
        normal: "9",
        shifted: Some("("),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "0",
        width: 2,
        normal: "0",
        shifted: Some(")"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "minus",
        width: 2,
        normal: "-",
        shifted: Some("_"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "equal",
        width: 2,
        normal: "=",
        shifted: Some("+"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "backspace",
        width: 5,
        normal: "⌫",
        shifted: None,
        backend: KeyBackend::Special("backspace"),
    },
];

const US_ROW_2: &[KeyDef] = &[
    KeyDef {
        id: "tab",
        width: 2,
        normal: "Tab",
        shifted: None,
        backend: KeyBackend::Special("tab"),
    },
    KeyDef {
        id: "q",
        width: 2,
        normal: "q",
        shifted: Some("Q"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "w",
        width: 2,
        normal: "w",
        shifted: Some("W"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "e",
        width: 2,
        normal: "e",
        shifted: Some("E"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "r",
        width: 2,
        normal: "r",
        shifted: Some("R"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "t",
        width: 2,
        normal: "t",
        shifted: Some("T"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "y",
        width: 2,
        normal: "y",
        shifted: Some("Y"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "u",
        width: 2,
        normal: "u",
        shifted: Some("U"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "i",
        width: 2,
        normal: "i",
        shifted: Some("I"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "o",
        width: 2,
        normal: "o",
        shifted: Some("O"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "p",
        width: 2,
        normal: "p",
        shifted: Some("P"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "lbrace",
        width: 2,
        normal: "[",
        shifted: Some("{"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "rbrace",
        width: 2,
        normal: "]",
        shifted: Some("}"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "backslash",
        width: 4,
        normal: "\\",
        shifted: Some("|"),
        backend: KeyBackend::Text,
    },
];

const US_ROW_3: &[KeyDef] = &[
    KeyDef {
        id: "caps",
        width: 3,
        normal: "Caps",
        shifted: None,
        backend: KeyBackend::Special("capslock"),
    },
    KeyDef {
        id: "a",
        width: 2,
        normal: "a",
        shifted: Some("A"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "s",
        width: 2,
        normal: "s",
        shifted: Some("S"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "d",
        width: 2,
        normal: "d",
        shifted: Some("D"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "f",
        width: 2,
        normal: "f",
        shifted: Some("F"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "g",
        width: 2,
        normal: "g",
        shifted: Some("G"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "h",
        width: 2,
        normal: "h",
        shifted: Some("H"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "j",
        width: 2,
        normal: "j",
        shifted: Some("J"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "k",
        width: 2,
        normal: "k",
        shifted: Some("K"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "l",
        width: 2,
        normal: "l",
        shifted: Some("L"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "semicolon",
        width: 2,
        normal: ";",
        shifted: Some(":"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "apostrophe",
        width: 2,
        normal: "'",
        shifted: Some("\""),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "enter",
        width: 5,
        normal: "⏎",
        shifted: None,
        backend: KeyBackend::Special("return"),
    },
];

const US_ROW_4: &[KeyDef] = &[
    KeyDef {
        id: "shift_l",
        width: 4,
        normal: "Shift",
        shifted: None,
        backend: KeyBackend::Modifier("shift"),
    },
    KeyDef {
        id: "z",
        width: 2,
        normal: "z",
        shifted: Some("Z"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "x",
        width: 2,
        normal: "x",
        shifted: Some("X"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "c",
        width: 2,
        normal: "c",
        shifted: Some("C"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "v",
        width: 2,
        normal: "v",
        shifted: Some("V"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "b",
        width: 2,
        normal: "b",
        shifted: Some("B"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "n",
        width: 2,
        normal: "n",
        shifted: Some("N"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "m",
        width: 2,
        normal: "m",
        shifted: Some("M"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "comma",
        width: 2,
        normal: ",",
        shifted: Some("<"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "dot",
        width: 2,
        normal: ".",
        shifted: Some(">"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "slash",
        width: 2,
        normal: "/",
        shifted: Some("?"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "shift_r",
        width: 4,
        normal: "Shift",
        shifted: None,
        backend: KeyBackend::Modifier("shift"),
    },
];

const US_ROW_5: &[KeyDef] = &[
    KeyDef {
        id: "ctrl_l",
        width: 2,
        normal: "Ctrl",
        shifted: None,
        backend: KeyBackend::Modifier("ctrl"),
    },
    KeyDef {
        id: "alt_l",
        width: 2,
        normal: "Alt",
        shifted: None,
        backend: KeyBackend::Modifier("alt"),
    },
    KeyDef {
        id: "space",
        width: 12,
        normal: "Space",
        shifted: None,
        backend: KeyBackend::Special("space"),
    },
    KeyDef {
        id: "alt_r",
        width: 2,
        normal: "Alt",
        shifted: None,
        backend: KeyBackend::Modifier("alt"),
    },
    KeyDef {
        id: "ctrl_r",
        width: 2,
        normal: "Ctrl",
        shifted: None,
        backend: KeyBackend::Modifier("ctrl"),
    },
    KeyDef {
        id: "left",
        width: 2,
        normal: "←",
        shifted: None,
        backend: KeyBackend::Special("left"),
    },
    KeyDef {
        id: "right",
        width: 2,
        normal: "→",
        shifted: None,
        backend: KeyBackend::Special("right"),
    },
    KeyDef {
        id: "down",
        width: 2,
        normal: "↓",
        shifted: None,
        backend: KeyBackend::Special("down"),
    },
    KeyDef {
        id: "up",
        width: 2,
        normal: "↑",
        shifted: None,
        backend: KeyBackend::Special("up"),
    },
];

const PT_ROW_1: &[KeyDef] = &[
    KeyDef {
        id: "esc",
        width: 2,
        normal: "Esc",
        shifted: None,
        backend: KeyBackend::Special("escape"),
    },
    KeyDef {
        id: "1",
        width: 2,
        normal: "1",
        shifted: Some("!"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "2",
        width: 2,
        normal: "2",
        shifted: Some("\""),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "3",
        width: 2,
        normal: "3",
        shifted: Some("#"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "4",
        width: 2,
        normal: "4",
        shifted: Some("$"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "5",
        width: 2,
        normal: "5",
        shifted: Some("%"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "6",
        width: 2,
        normal: "6",
        shifted: Some("&"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "7",
        width: 2,
        normal: "7",
        shifted: Some("/"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "8",
        width: 2,
        normal: "8",
        shifted: Some("("),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "9",
        width: 2,
        normal: "9",
        shifted: Some(")"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "0",
        width: 2,
        normal: "0",
        shifted: Some("="),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "apostrophe_key",
        width: 2,
        normal: "'",
        shifted: Some("?"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "double_angle",
        width: 2,
        normal: "«",
        shifted: Some("»"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "backspace",
        width: 5,
        normal: "⌫",
        shifted: None,
        backend: KeyBackend::Special("backspace"),
    },
];

const PT_ROW_2: &[KeyDef] = &[
    KeyDef {
        id: "tab",
        width: 2,
        normal: "Tab",
        shifted: None,
        backend: KeyBackend::Special("tab"),
    },
    KeyDef {
        id: "q",
        width: 2,
        normal: "q",
        shifted: Some("Q"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "w",
        width: 2,
        normal: "w",
        shifted: Some("W"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "e",
        width: 2,
        normal: "e",
        shifted: Some("E"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "r",
        width: 2,
        normal: "r",
        shifted: Some("R"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "t",
        width: 2,
        normal: "t",
        shifted: Some("T"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "y",
        width: 2,
        normal: "y",
        shifted: Some("Y"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "u",
        width: 2,
        normal: "u",
        shifted: Some("U"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "i",
        width: 2,
        normal: "i",
        shifted: Some("I"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "o",
        width: 2,
        normal: "o",
        shifted: Some("O"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "p",
        width: 2,
        normal: "p",
        shifted: Some("P"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "plus",
        width: 2,
        normal: "+",
        shifted: Some("*"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "acute",
        width: 2,
        normal: "´",
        shifted: Some("`"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "backslash",
        width: 4,
        normal: "\\",
        shifted: Some("|"),
        backend: KeyBackend::Text,
    },
];

const PT_ROW_3: &[KeyDef] = &[
    KeyDef {
        id: "caps",
        width: 3,
        normal: "Caps",
        shifted: None,
        backend: KeyBackend::Special("capslock"),
    },
    KeyDef {
        id: "a",
        width: 2,
        normal: "a",
        shifted: Some("A"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "s",
        width: 2,
        normal: "s",
        shifted: Some("S"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "d",
        width: 2,
        normal: "d",
        shifted: Some("D"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "f",
        width: 2,
        normal: "f",
        shifted: Some("F"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "g",
        width: 2,
        normal: "g",
        shifted: Some("G"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "h",
        width: 2,
        normal: "h",
        shifted: Some("H"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "j",
        width: 2,
        normal: "j",
        shifted: Some("J"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "k",
        width: 2,
        normal: "k",
        shifted: Some("K"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "l",
        width: 2,
        normal: "l",
        shifted: Some("L"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "cedilla",
        width: 2,
        normal: "ç",
        shifted: Some("Ç"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "masculine",
        width: 2,
        normal: "º",
        shifted: Some("ª"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "enter",
        width: 5,
        normal: "⏎",
        shifted: None,
        backend: KeyBackend::Special("return"),
    },
];

const PT_ROW_4: &[KeyDef] = &[
    KeyDef {
        id: "shift_l",
        width: 4,
        normal: "Shift",
        shifted: None,
        backend: KeyBackend::Modifier("shift"),
    },
    KeyDef {
        id: "angle",
        width: 2,
        normal: "<",
        shifted: Some(">"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "z",
        width: 2,
        normal: "z",
        shifted: Some("Z"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "x",
        width: 2,
        normal: "x",
        shifted: Some("X"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "c",
        width: 2,
        normal: "c",
        shifted: Some("C"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "v",
        width: 2,
        normal: "v",
        shifted: Some("V"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "b",
        width: 2,
        normal: "b",
        shifted: Some("B"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "n",
        width: 2,
        normal: "n",
        shifted: Some("N"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "m",
        width: 2,
        normal: "m",
        shifted: Some("M"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "comma",
        width: 2,
        normal: ",",
        shifted: Some(";"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "dot",
        width: 2,
        normal: ".",
        shifted: Some(":"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "dash",
        width: 2,
        normal: "-",
        shifted: Some("_"),
        backend: KeyBackend::Text,
    },
    KeyDef {
        id: "shift_r",
        width: 4,
        normal: "Shift",
        shifted: None,
        backend: KeyBackend::Modifier("shift"),
    },
];

const PT_ROW_5: &[KeyDef] = &[
    KeyDef {
        id: "ctrl_l",
        width: 2,
        normal: "Ctrl",
        shifted: None,
        backend: KeyBackend::Modifier("ctrl"),
    },
    KeyDef {
        id: "alt_l",
        width: 2,
        normal: "Alt",
        shifted: None,
        backend: KeyBackend::Modifier("alt"),
    },
    KeyDef {
        id: "space",
        width: 12,
        normal: "Space",
        shifted: None,
        backend: KeyBackend::Special("space"),
    },
    KeyDef {
        id: "altgr",
        width: 2,
        normal: "AltGr",
        shifted: None,
        backend: KeyBackend::Modifier("altgr"),
    },
    KeyDef {
        id: "ctrl_r",
        width: 2,
        normal: "Ctrl",
        shifted: None,
        backend: KeyBackend::Modifier("ctrl"),
    },
    KeyDef {
        id: "left",
        width: 2,
        normal: "←",
        shifted: None,
        backend: KeyBackend::Special("left"),
    },
    KeyDef {
        id: "right",
        width: 2,
        normal: "→",
        shifted: None,
        backend: KeyBackend::Special("right"),
    },
    KeyDef {
        id: "down",
        width: 2,
        normal: "↓",
        shifted: None,
        backend: KeyBackend::Special("down"),
    },
    KeyDef {
        id: "up",
        width: 2,
        normal: "↑",
        shifted: None,
        backend: KeyBackend::Special("up"),
    },
];

const US_ROWS: &[&[KeyDef]] = &[US_ROW_1, US_ROW_2, US_ROW_3, US_ROW_4, US_ROW_5];
const PT_ROWS: &[&[KeyDef]] = &[PT_ROW_1, PT_ROW_2, PT_ROW_3, PT_ROW_4, PT_ROW_5];

pub const US_LAYOUT: Layout = Layout {
    code: "US",
    name: "English (US)",
    xkb_layout: "us",
    rows: US_ROWS,
};

pub const PT_LAYOUT: Layout = Layout {
    code: "PT",
    name: "Portuguese (PT)",
    xkb_layout: "pt",
    rows: PT_ROWS,
};

pub const ALL_LAYOUTS: &[Layout] = &[US_LAYOUT, PT_LAYOUT];

pub fn layout_by_code(code: &str) -> Layout {
    ALL_LAYOUTS
        .iter()
        .copied()
        .find(|layout| layout.code.eq_ignore_ascii_case(code))
        .unwrap_or(US_LAYOUT)
}

#[cfg(test)]
mod tests {
    use super::{layout_by_code, PT_LAYOUT, US_LAYOUT};

    #[test]
    fn lookup_is_case_insensitive() {
        assert_eq!(layout_by_code("pt").code, PT_LAYOUT.code);
    }

    #[test]
    fn lookup_falls_back_to_us() {
        assert_eq!(layout_by_code("unknown").code, US_LAYOUT.code);
    }
}
