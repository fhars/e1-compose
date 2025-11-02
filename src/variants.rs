use crate::utils;

fn rules(key: &str, ch: char, offset: u32) {
    print!("<dead_breve> <{}> <{}>:", key, ch);
    let replacement = char::from_u32(offset + (ch as u32)).unwrap();
    utils::print_rule(&format!("{}", replacement))
}

pub fn run() {
    let superscripts = [
        ("a", "ᵃ"),
        ("b", "ᵇ"),
        ("c", "ᶜ"),
        ("d", "ᵈ"),
        ("e", "ᵉ"),
        ("f", "ᶠ"),
        ("g", "ᵍ"),
        ("h", "ʰ"),
        ("i", "ⁱ"),
        ("j", "ʲ"),
        ("k", "ᵏ"),
        ("l", "ˡ"),
        ("m", "ᵐ"),
        ("n", "ⁿ"),
        ("o", "ᵒ"),
        ("p", "ᵖ"),
        ("r", "ʳ"),
        ("s", "ˢ"),
        ("t", "ᵗ"),
        ("u", "ᵘ"),
        ("v", "ᵛ"),
        ("w", "ʷ"),
        ("x", "ˣ"),
        ("y", "ʸ"),
        ("z", "ᶻ"),
        ("A", "ᴬ"),
        ("B", "ᴮ"),
        ("D", "ᴰ"),
        ("E", "ᴱ"),
        ("G", "ᴳ"),
        ("H", "ᴴ"),
        ("I", "ᴵ"),
        ("J", "ᴶ"),
        ("K", "ᴷ"),
        ("L", "ᴸ"),
        ("M", "ᴹ"),
        ("N", "ᴺ"),
        ("O", "ᴼ"),
        ("P", "ᴾ"),
        ("R", "ᴿ"),
        ("T", "ᵀ"),
        ("U", "ᵁ"),
        ("V", "ⱽ"),
        ("W", "ᵂ"),
    ];
    let subscripts = [
        ("a", "ₐ"),
        ("e", "ₑ"),
        ("h", "ₕ"),
        ("i", "ᵢ"),
        ("j", "ⱼ"),
        ("k", "ₖ"),
        ("l", "ₗ"),
        ("m", "ₘ"),
        ("n", "ₙ"),
        ("o", "ₒ"),
        ("p", "ₚ"),
        ("r", "ᵣ"),
        ("s", "ₛ"),
        ("t", "ₜ"),
        ("u", "ᵤ"),
        ("v", "ᵥ"),
        ("x", "ₓ"),
        // The following will be added in the future:
        // https://www.unicode.org/L2/L2024/24268-n5291-post-17-chart.pdf
        // ("w", "₝"),
        // ("y", "₞"),
        // ("z", "₟"),
    ];

    for (key, replacement) in superscripts {
        print!("<dead_breve> <dead_circumflex> <{}>:", key);
        utils::print_rule(replacement)
    }
    for (key, replacement) in subscripts {
        print!("<dead_breve> <dead_caron> <{}>:", key);
        utils::print_rule(replacement)
    }
    for ch in 'a'..='z' {
        let uc = ch.to_ascii_uppercase();
        rules("dead_abovering", uc, 0x24b6 - ('A' as u32));
        rules("dead_abovering", ch, 0x24d0 - ('a' as u32));
    }
}
