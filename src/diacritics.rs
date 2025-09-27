use crate::utils;

pub fn run() {
    let chars = [
        ("◌̐", "dead_abovedot", "dead_breve"),
        ("◌̓", "dead_horn", "ssharp"),
        ("◌̭", "dead_caron", "dead_circumflex"), // TODO: this should be a dead key for Ḓ/ḓ/Ḙ/ḙ/Ḽ/ḽ/Ṋ/ṋ/Ṱ/ṱ/Ṷ/ṷ, not a combining mark...
        ("◌̥", "dead_caron", "dead_abovering"),
        ("◌͟◌", "dead_caron", "dead_belowmacron"),
        ("◌͜◌", "dead_caron", "dead_breve"),
        ("◌̭", "dead_belowcomma", "dead_circumflex"),
        ("◌̍", "dead_belowcomma", "dead_acute"),
        ("◌̏", "dead_belowcomma", "dead_grave"),
        ("◌̅", "dead_belowcomma", "dead_macron"),
        ("◌̯", "dead_belowcomma", "dead_breve"),
        ("◌͘", "dead_belowcomma", "dead_abovedot"),
        ("◌̒", "dead_belowcomma", "dead_hook"),
        ("◌̕", "dead_belowcomma", "dead_horn"),
        ("◌̲", "dead_belowcomma", "dead_belowmacron"),
        ("◌᪷", "dead_belowcomma", "dead_ogonek"),
    ];
    let deadkeys = [
        ("dead_circumflex", "◌̂"),
        ("dead_acute", "◌́"),
        ("dead_grave", "◌̀"),
        ("dead_abovedot", "◌̇"),
        ("dead_belowmacron", "◌̱"),
        ("dead_cedilla", "◌̧"),
        ("dead_belowcomma", "◌̦"),
        ("dead_ogonek", "◌̨"),
        ("dead_belowdot", "◌̣"),
        ("dead_stroke", "◌̵"),
    ];
    for (replacement, first, second) in chars {
        print!("<{}> <{}>:", first, second);
        utils::print_rule(replacement);
    }
    for (key, combiner) in deadkeys {
        print!("<{}> <{}>:", key, key);
        utils::print_rule(combiner);
    }
}
