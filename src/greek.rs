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
        "odiaeresis",
        "p",
        "q",
        "r",
        "s",
        "t",
        "udiaeresis",
        "v",
        "w",
        "x",
        "y",
        "z",
        "ssharp",
    ];
    let chars = [
        [
            "Α", "Β", "Ψ", "Δ", "Ε", "Φ", "Γ", "Η", "Ι", "Ξ", "Κ", "Λ", "Μ", "Ν", "Ο", "Π", "◌͂",
            "Ρ", "Σ", "Τ", "Θ", "Ω", "◌̔", "Χ", "Υ", "Ζ", "",
        ],
        [
            "α", "β", "ψ", "δ", "ε", "φ", "γ", "η", "ι", "ξ", "κ", "λ", "μ", "ν", "ο", "π", "◌ͅ",
            "ρ", "σ", "τ", "θ", "ω", "ς", "χ", "υ", "ζ", "◌̓",
        ],
    ];

    let vowels = [
        (0, "ά"),
        (4, "έ"),
        (8, "ί"),
        (14, "ό"),
        (21, "ώ"),
        (24, "ύ"),
    ];
    let dia_vowels = [(8, "ΐ"), (24, "ΰ")];
    for (ix, key) in keys.iter().enumerate() {
        if !chars[0][ix].is_empty() {
            print!("<dead_horn> <{}>:", key.to_uppercase());
            utils::print_rule(&chars[0][ix].to_uppercase());
        }
        if !chars[1][ix].is_empty() {
            print!("<dead_horn> <{}>:", key);
            utils::print_rule(chars[1][ix]);
        }
    }
    for (key, replacement) in vowels {
        print!("<dead_horn> <dead_acute> <{}>:", keys[key].to_uppercase());
        utils::print_rule(&replacement.to_uppercase());
        print!("<dead_horn> <dead_acute> <{}>:", keys[key]);
        utils::print_rule(replacement);
    }
    for (key, replacement) in dia_vowels {
        print!("<dead_horn> <dead_breve> <{}>:", keys[key]);
        utils::print_rule(replacement);
    }
}
