use std::path::Path;
use std::sync::LazyLock;

use crate::file::display_path;
use console::{Color, Style, StyledObject, style};

pub fn estyle<D>(val: D) -> StyledObject<D> {
    style(val).for_stderr()
}

pub fn ecyan<D>(val: D) -> StyledObject<D> {
    estyle(val).cyan()
}

pub fn eblue<D>(val: D) -> StyledObject<D> {
    estyle(val).blue()
}

pub fn emagenta<D>(val: D) -> StyledObject<D> {
    estyle(val).magenta()
}

pub fn egreen<D>(val: D) -> StyledObject<D> {
    estyle(val).green()
}

pub fn eyellow<D>(val: D) -> StyledObject<D> {
    estyle(val).yellow()
}

pub fn ered<D>(val: D) -> StyledObject<D> {
    estyle(val).red()
}

pub fn eblack<D>(val: D) -> StyledObject<D> {
    estyle(val).black()
}

pub fn eunderline<D>(val: D) -> StyledObject<D> {
    estyle(val).underlined()
}

pub fn edim<D>(val: D) -> StyledObject<D> {
    estyle(val).dim()
}

pub fn ebold<D>(val: D) -> StyledObject<D> {
    estyle(val).bold()
}

pub fn nbold<D>(val: D) -> StyledObject<D> {
    nstyle(val).bold()
}

pub fn epath(path: &Path) -> StyledObject<String> {
    estyle(display_path(path))
}

pub fn nstyle<D>(val: D) -> StyledObject<D> {
    style(val).for_stdout()
}

pub fn ncyan<D>(val: D) -> StyledObject<D> {
    nstyle(val).cyan()
}

pub fn nunderline<D>(val: D) -> StyledObject<D> {
    nstyle(val).underlined()
}

pub fn nyellow<D>(val: D) -> StyledObject<D> {
    nstyle(val).yellow()
}

pub fn nred<D>(val: D) -> StyledObject<D> {
    nstyle(val).red()
}

pub fn ndim<D>(val: D) -> StyledObject<D> {
    nstyle(val).dim()
}

pub fn nbright<D>(val: D) -> StyledObject<D> {
    nstyle(val).bright()
}

fn prefix_style(hash_key: impl AsRef<str>, stderr: bool) -> Style {
    static COLORS: LazyLock<Vec<Color>> =
        LazyLock::new(|| vec![Color::Blue, Color::Magenta, Color::Cyan, Color::Green]);

    let hash = hash_key.as_ref().chars().map(|c| c as usize).sum::<usize>();
    let styled = Style::new().fg(COLORS[hash % COLORS.len()]);
    let mut styled = if stderr {
        styled.for_stderr()
    } else {
        styled.for_stdout()
    };
    match (hash / COLORS.len()) % 4 {
        1 => styled = styled.bold(),
        2 => styled = styled.dim(),
        3 => styled = styled.bright(),
        _ => {}
    }

    styled
}

pub fn prefix(label: impl Into<String>, hash_key: impl AsRef<str>, stderr: bool) -> String {
    prefix_style(hash_key, stderr)
        .apply_to(label.into())
        .to_string()
}

/// Return the ANSI sequence that starts the style used for a task prefix.
///
/// Styling is forced here because the caller separately decides whether color
/// is enabled for the selected output mode. The trailing reset emitted by
/// `console` is removed so tasks can apply the style to their own output.
pub fn prefix_ansi(hash_key: impl AsRef<str>) -> String {
    let rendered = prefix_style(hash_key, false)
        .force_styling(true)
        .apply_to("")
        .to_string();
    rendered
        .strip_suffix("\x1b[0m")
        .expect("forced prefix style should end with an ANSI reset")
        .to_string()
}

/// Return the reset to print after a line relayed from a child process.
///
/// A line that sets a style without clearing it would carry that style into
/// the next line and its prefix, so a line with an escape sequence ends with
/// a reset. A plain line gets none, so output such as JSON reaches a pipe
/// unchanged even when color is forced.
pub fn line_reset(line: &str, colors: bool) -> &'static str {
    if colors && line.contains("\x1b[") {
        "\x1b[0m"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_reset_follows_only_styled_lines() {
        assert_eq!(line_reset(r#"{"a":1}"#, true), "");
        assert_eq!(line_reset("\x1b[31mred", true), "\x1b[0m");
        assert_eq!(line_reset("\x1b[1mbold\x1b[0m", true), "\x1b[0m");
        assert_eq!(line_reset("\x1b[31mred", false), "");
    }

    #[test]
    fn prefix_ansi_matches_rendered_prefix_style() {
        for name in ["a", "b", "c", "d", "alpha", "bravo", "charlie", "echo"] {
            let ansi = prefix_ansi(name);
            let rendered = prefix_style(name, false)
                .force_styling(true)
                .apply_to("label")
                .to_string();
            assert_eq!(rendered, format!("{ansi}label\x1b[0m"));
        }
    }

    #[test]
    fn prefix_ansi_includes_color_and_emphasis() {
        assert_eq!(prefix_ansi("p"), "\x1b[34m");
        assert_eq!(prefix_ansi("a"), "\x1b[35m");
        assert_eq!(prefix_ansi("b"), "\x1b[36m");
        assert_eq!(prefix_ansi("c"), "\x1b[32m");
        assert_eq!(prefix_ansi("alpha"), "\x1b[36m\x1b[1m");
        assert_eq!(prefix_ansi("bravo"), "\x1b[36m\x1b[2m");
        assert_eq!(prefix_ansi("echo"), "\x1b[38;5;10m");
    }
}
