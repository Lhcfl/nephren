// Source - https://stackoverflow.com/a/79614957
// Posted by roylaurie
// Retrieved 2026-05-25, License - CC BY-SA 4.0

use clap::builder::styling::{AnsiColor, Effects, Style, Styles};

//pub(crate) const NOP: Style = Style::new();
const HEADER: Style = AnsiColor::Green
    .on_default()
    .effects(Effects::BOLD.insert(Effects::UNDERLINE));
const USAGE: Style = Style::new().effects(Effects::BOLD);
const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
//const WARN: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
//const NOTE: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
//const GOOD: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

/// Cargo's color style
/// [source](https://github.com/crate-ci/clap-cargo/blob/master/src/style.rs)
pub const CARGO_STYLING: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);
