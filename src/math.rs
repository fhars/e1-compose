// IPA-Buchstaben: erst Buchstabe (Spaltenauswahl) – dann Ziffer (Zeilenauswahl)

use crate::utils;

pub fn run() {
    let keys = [
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "g",
        "h",
        "i",
        "j",
        "k",
        "l",
        "m",
        "n",
        "o",
        "p",
        "q",
        "r",
        "s",
        "t",
        "u",
        "v",
        "w",
        "x",
        "y",
        "z",
        "adiaeresis",
        "odiaeresis",
        "udiaeresis",
        "ssharp",
        // ×  /  (  )  [  ]  =  +  *  ~  #  <  >  .  ;  :  ·  -
        "multiply",
        "slash",
        "parenleft",
        "parenright",
        "bracketleft",
        "bracketright",
        "equal",
        "plus",
        "asterisk",
        "asciitilde",
        "numbersign",
        "less",
        "greater",
        "period",
        "semicolon",
        "colon",
        "periodcentered",
        "minus",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "0",
    ];
    let chars = [
        [
            "Å", "⇎", "ℂ", "∆", "∄", "∴", "⊄", "ℏ", "∝", "⊅", "⊇", "∩", "⇏", "ℕ", "∠", "℗", "ℚ",
            "ℝ", "∇", "∟", "Ů", "∧", "ʬ", "ℵ", "∢", "ℤ", "∌", "∉", "∦", "", "", "", "", "", "", "",
            "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        ],
        [
            "å", "⇔", "∛", "∂", "∃", "∵", "⊂", "ℎ", "∞", "⊃", "⊆", "∪", "⇒", "⇐", "∡", "∏", "∀",
            "√", "↔", "⊥", "ů", "∨", "ẘ", "✕", "ẙ", "⚡︎", "∋", "∈", "∥", "∫", "⊗", "∕", "❲", "❳",
            "⟦", "⟧", "≝", "⊕", "∗", "⸛", "∎", "≪", "≫", "⋅", "⇌", "∶", "∙", "⊖", "①", "②", "③",
            "④", "⑤", "⑥", "⑦", "⑧", "⑨", "⑩",
        ],
    ];

    for (ix, key) in keys.iter().enumerate() {
        if !chars[0][ix].is_empty() {
            print!("<dead_abovering> <{}>:", &key.to_uppercase());
            utils::print_rule(chars[0][ix]);
        }
        if !chars[1][ix].is_empty() {
            print!("<dead_abovering> <{}>:", key);
            utils::print_rule(chars[1][ix]);
        }
    }
    for (key, replacement) in [("1", "⅓"), ("2", "⅔")] {
        print!("<dead_breve> <{}>:", key);
        utils::print_rule(replacement);
    }
    for (key, replacement) in [("1", "⅛"), ("3", "⅜"), ("5", "⅝"), ("7", "⅞")] {
        print!("<dead_horn> <{}>:", key);
        utils::print_rule(replacement);
    }

    for (key, replacement) in [
        ("minus", "⹀"),
        ("equal", "≡"),
        ("plus", "±"),
        ("less", "≤"),
        ("greater", "≥"),
    ] {
        print!("<dead_belowmacron> <{}>:", key);
        utils::print_rule(replacement);
    }
}
