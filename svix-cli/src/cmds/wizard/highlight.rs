//! Syntax highlighting for the code samples, drawn on the theme's own background so
//! they read the same whether the terminal is light or dark.

use std::{
    cmp::Reverse,
    sync::{Arc, OnceLock},
};

use arborium::{
    theme::{builtin, Theme},
    GrammarStore, Highlighter,
};
use arborium_theme::{capture_to_slot, slot_to_highlight_index};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

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
    fn extension(self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::Go => "go",
            Self::Rust => "rs",
            Self::Java => "java",
            Self::Kotlin => "kt",
            Self::CSharp => "cs",
            Self::Ruby => "rb",
            Self::Php => "php",
            Self::Shell => "sh",
        }
    }
}

struct Engine {
    store: Arc<GrammarStore>,
    theme: Theme,
}

fn engine() -> &'static Engine {
    static ENGINE: OnceLock<Engine> = OnceLock::new();

    ENGINE.get_or_init(|| Engine {
        store: Arc::new(GrammarStore::new()),
        theme: builtin::one_dark(),
    })
}

fn colour(colour: arborium::theme::Color) -> Color {
    Color::Rgb(colour.r, colour.g, colour.b)
}

/// The background the samples are drawn on, for the box around them to match.
pub(super) fn background() -> Style {
    match engine().theme.background {
        Some(background) => Style::new().bg(colour(background)),
        None => Style::new(),
    }
}

fn style(index: Option<usize>) -> Style {
    let theme = &engine().theme;
    let fg = index
        .and_then(|index| theme.style(index))
        .and_then(|style| style.fg)
        .or(theme.foreground);

    match fg {
        Some(fg) => background().fg(colour(fg)),
        None => background(),
    }
}

/// Highlights `code`, falling back to unstyled lines if the language won't parse.
pub(super) fn highlight(syntax: Syntax, code: &str) -> Vec<Line<'static>> {
    let mut spans = Highlighter::with_store(engine().store.clone())
        .highlight_spans(syntax.extension(), code)
        .unwrap_or_default();
    spans.sort_by_key(|span| (span.start, Reverse(span.end), span.pattern_index));

    let mut styles = vec![None; code.len()];
    for span in spans {
        if let (Some(index), Some(range)) = (
            slot_to_highlight_index(capture_to_slot(&span.capture)),
            styles.get_mut(span.start as usize..span.end as usize),
        ) {
            range.fill(Some(index));
        }
    }

    code.lines()
        .map(|line| {
            let offset = line.as_ptr() as usize - code.as_ptr() as usize;
            let mut rest = line;
            styles[offset..offset + line.len()]
                .chunk_by(|a, b| a == b)
                .map(|run| {
                    let (text, tail) = rest.split_at(run.len());
                    rest = tail;
                    Span::styled(text.to_owned(), style(run[0]))
                })
                .collect()
        })
        .collect()
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
                engine().store.get(syntax.extension()).is_some(),
                "no syntax for .{}",
                syntax.extension()
            );
        }
    }
}
