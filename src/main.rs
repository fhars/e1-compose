mod diacritics;
mod greek;
mod ipa;
mod letters;
mod math;
mod other;
mod utils;
mod variants;

fn main() {
    ipa::run();
    greek::run();
    math::run();
    letters::run();
    other::run();
    diacritics::run();
    variants::run();
}
