# XCompose rules for the DIN 2137 E1 Keyboard Layout

The E1 layout definition distributed as part of XKB does not implement
all parts of the layout, only the `Gr` and `Ex` layers and some of the
dead key rules work. The code here tries to implement most of the
remaining functionality as XCompose rules. Most of the tables have
been created form the data of the [german Wikipedia page for the E1
Layout](https://de.wikipedia.org/wiki/E1_(Tastaturbelegung)),

If you just want to use it, just copy the generated `XCompose` file
included here to your `${HOME}/.XCompose`. If you want to recreate it
or change the generator, you need a rust intallation and run `cargo
run` in a copy of this repo.

## What does and doesn't work?

### IPA

Entering IPA characters works, all the rules that should create
combining diacritical marks so not work.

### Greek

Entering greek characters works: αβγΔ.

### Ancient letters and linguistic characters

Works: Ƿȥ℞

### Mathematical characters

Works: ∛ℏ⊗ℵ

### Miscellaneaous characters

Works: ⌘❦※

Also, enclosed numbers (with `AltGr-O` `<Number>`) work: ①②③

### Other diacritical characters

None of the combining diacritical characters that use rules with just
two dead characters work.

### Enclosed, sub- and superscripted letters.

Enclosed letters and the sub- and superscript letters that exist in
Unicode work.
