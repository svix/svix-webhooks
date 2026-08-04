//! Syntax highlighting for the code samples, drawn on the theme's own background so
//! they read the same whether the terminal is light or dark.

use std::sync::OnceLock;

use ratatui::{
    style::{Color, Style},
    text::Line,
};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use tui_syntax_highlight::Highlighter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Syntax {
    Python,
    JavaScript,
    Go,
    Rust,
    Java,
    Kotlin,
    CSharp,
    Ruby,
    Php,
    Shell,
}

impl Syntax {
    /// Looked up by extension, which is stabler than syntect's display names.
    fn extension(self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::Go => "go",
            Self::Rust => "rs",
            Self::Java => "java",
            // syntect ships no Kotlin grammar; Java's is close enough.
            Self::Kotlin => "java",
            Self::CSharp => "cs",
            Self::Ruby => "rb",
            Self::Php => "php",
            Self::Shell => "sh",
        }
    }
}

/// The syntax and theme dumps are slow to load, so this is built once on first use.
struct Engine {
    syntaxes: SyntaxSet,
    highlighter: Highlighter,
    background: Option<Color>,
}

/// A dark theme on its own background: readable on any terminal, close to the docs.
const THEME: &str = "base16-ocean.dark";

fn engine() -> &'static Engine {
    static ENGINE: OnceLock<Engine> = OnceLock::new();

    ENGINE.get_or_init(|| {
        let theme = ThemeSet::load_defaults().themes[THEME].clone();
        let highlighter = Highlighter::new(theme).line_numbers(false);

        Engine {
            syntaxes: SyntaxSet::load_defaults_newlines(),
            background: highlighter.get_background_color(),
            highlighter,
        }
    })
}

/// The background the samples are drawn on, for the box around them to match.
pub(super) fn background() -> Style {
    match engine().background {
        Some(background) => Style::new().bg(background),
        None => Style::new(),
    }
}

/// Highlights `code`, falling back to unstyled lines if the syntax or theme won't load.
pub(super) fn highlight(syntax: Syntax, code: &str) -> Vec<Line<'static>> {
    let engine = engine();
    let plain = || {
        code.lines()
            .map(|line| Line::styled(line.to_owned(), background()))
            .collect::<Vec<_>>()
    };

    let Some(reference) = engine.syntaxes.find_syntax_by_extension(syntax.extension()) else {
        return plain();
    };

    match engine
        .highlighter
        .highlight_lines(code.lines(), reference, &engine.syntaxes)
    {
        Ok(text) => text.lines,
        Err(_) => plain(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_keep_their_text_and_gain_colour() {
        let code = "# a comment\nsvix = Svix(\"token\")";
        let lines = highlight(Syntax::Python, code);

        let rendered: Vec<String> = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect()
            })
            .collect();
        assert_eq!(rendered, code.lines().collect::<Vec<_>>());

        let styles: Vec<Style> = lines
            .iter()
            .flat_map(|line| line.spans.iter().map(|span| span.style))
            .collect();
        assert!(
            styles.iter().filter(|s| s.fg.is_some()).count() > 1,
            "the sample should be highlighted, not one flat colour"
        );
    }

    #[test]
    fn every_language_the_wizard_shows_has_a_syntax() {
        for syntax in [
            Syntax::Python,
            Syntax::JavaScript,
            Syntax::Go,
            Syntax::Rust,
            Syntax::Java,
            Syntax::Kotlin,
            Syntax::CSharp,
            Syntax::Ruby,
            Syntax::Php,
            Syntax::Shell,
        ] {
            assert!(
                engine()
                    .syntaxes
                    .find_syntax_by_extension(syntax.extension())
                    .is_some(),
                "no syntax for .{}",
                syntax.extension()
            );
        }
    }
}
