use crate::utils;

pub fn run() {
    let chars = [
        ("1", "ǀ"),
        ("'", "ǀ"),
        ("2", "ǁ"),
        ("\"", "ǁ"),
        ("3", "ǂ"),
        ("#", "ǂ"),
        ("4", "ǃ"),
        ("!", "ǃ"),
        ("b", "ꜣ"),
        ("c", "ꜥ"),
        ("i", "ꞽ"),
        ("f", "ƒ"),
        ("h", "ƕ"),
        ("q", "ɂ"),
        ("r", "ɼ"),
        ("w", "ƿ"),
        ("y", "ȝ"),
        ("z", "ȥ"),
        ("B", "Ꜣ"),
        ("C", "Ꜥ"),
        ("I", "Ꞽ"),
        ("F", "Ƒ"),
        ("H", "Ƕ"),
        ("Q", "Ɂ"),
        ("R", "℞"),
        ("W", "Ƿ"),
        ("Y", "Ȝ"),
        ("Z", "Ȥ"),
        ("parenleft", "⸤"),
        ("parenright", "⸥"),
        ("bracketleft", "⸢"),
        ("bracketright", "⸣"),
        ("equal", "⸗"),
        ("bar", "⦀"),
        ("question", "⸮"),
        ("minutes", "‵"),
        ("seconds", "‶"),
    ];

    for (key, replacement) in chars {
        print!("<dead_belowcomma> <{}>:", key);
        utils::print_rule(replacement);
    }
}
