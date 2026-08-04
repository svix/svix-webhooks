//! Drawing helpers shared across the quickstart's steps.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::cmds::wizard::highlight::{background, highlight, Syntax};

// Prose and code keep the terminal's own foreground colour, and anything secondary is
// dimmed rather than greyed: a fixed grey is unreadable on a light background.
pub(super) const HEADING: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
pub(super) const VALUE: Style = Style::new().fg(Color::Green);
pub(super) const DIM: Style = Style::new().add_modifier(Modifier::DIM);

/// Prose in the terminal's own colours; the paragraph widget wraps it.
pub(super) fn prose(text: &str) -> Line<'static> {
    Line::raw(text.to_owned())
}

/// A `label: value` line, with the value in the colour used for things worth copying.
pub(super) fn field(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label}: "), DIM),
        Span::styled(value.to_owned(), VALUE),
    ])
}

/// A pick-one list, rendered inline so it scrolls with the rest of the step.
pub(super) fn choices(options: &[&'static str], selected: usize) -> Vec<Line<'static>> {
    options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            if index == selected {
                Line::from(vec![
                    Span::styled("› ", HEADING),
                    Span::styled(*option, HEADING),
                ])
            } else {
                Line::styled(format!("  {option}"), DIM)
            }
        })
        .collect()
}

/// Spaces between the box edges and the code.
const PADDING: usize = 2;

/// A highlighted code sample, drawn as text so it scrolls with everything else. No side
/// borders: selecting the sample with the mouse then picks up the code alone.
pub(super) fn sample(title: &str, syntax: Syntax, text: &str, width: u16) -> Vec<Line<'static>> {
    let width = (width as usize).max(8);
    let inner = width - 2 * PADDING;
    let rule = background().add_modifier(Modifier::DIM);

    let heading = format!("─ {title} ");
    let mut lines = vec![Line::styled(
        format!(
            "{heading}{}",
            "─".repeat(width.saturating_sub(heading.chars().count()))
        ),
        rule,
    )];

    lines.extend(
        highlight(syntax, text)
            .into_iter()
            .flat_map(|line| wrap(line, inner))
            .map(|line| {
                let mut spans = vec![Span::raw(" ".repeat(PADDING))];
                spans.extend(fit(line, inner).spans);
                spans.push(Span::raw(" ".repeat(PADDING)));
                Line::from(spans).style(background())
            }),
    );

    lines.push(Line::styled("─".repeat(width), rule));

    lines
}

/// Breaks a styled line into chunks at most `width` cells wide, keeping the spans'
/// styles, so long code lines wrap inside the box instead of being cut off.
fn wrap(line: Line<'static>, width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut current: Vec<Span> = Vec::new();
    let mut used = 0;

    for span in line.spans {
        let mut content = span.content.to_string();
        while !content.is_empty() {
            let taken: String = content.chars().take(width - used).collect();
            content = content.chars().skip(taken.chars().count()).collect();
            used += taken.chars().count();
            current.push(Span::styled(taken, span.style));

            if used == width {
                lines.push(Line::from(std::mem::take(&mut current)));
                used = 0;
            }
        }
    }
    if !current.is_empty() || lines.is_empty() {
        lines.push(Line::from(current));
    }

    lines
}

/// Truncates or pads a line to exactly `width` cells.
fn fit(line: Line<'static>, width: usize) -> Line<'static> {
    let mut spans = Vec::new();
    let mut used = 0;

    for span in line.spans {
        if used >= width {
            break;
        }
        let content = span.content.to_string();
        let len = content.chars().count();

        if used + len <= width {
            used += len;
            spans.push(Span::styled(content, span.style));
        } else {
            let kept: String = content.chars().take(width - used - 1).collect();
            spans.push(Span::styled(format!("{kept}…"), span.style));
            used = width;
        }
    }

    if used < width {
        spans.push(Span::raw(" ".repeat(width - used)));
    }

    Line::from(spans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_code_lines_wrap_inside_the_box_instead_of_truncating() {
        let code = "https://api.svix.com/api/v1/auth/app-portal-access/app_2abc123def456ghi789";
        let lines = sample("cURL", Syntax::Shell, code, 40);

        let rendered: String = lines
            .iter()
            .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
            .collect();
        assert!(!rendered.contains('…'), "nothing is cut off:\n{rendered}");
        assert!(
            rendered.replace(' ', "").contains("app_2abc123def456ghi789"),
            "the end of the long line is still there:\n{rendered}"
        );
        assert!(
            lines.len() > 3,
            "the long line takes more than one row: {lines:#?}"
        );
    }
}

/// Copies `text` through the terminal itself with OSC 52, which keeps working over SSH,
/// where a clipboard library would reach for the wrong machine's clipboard.
pub(super) fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
    use std::io::Write as _;

    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let sequence = format!("\x1b]52;c;{}\x07", STANDARD.encode(text));
    // tmux swallows escape sequences it isn't told to pass through.
    let sequence = match std::env::var_os("TMUX") {
        Some(_) => format!("\x1bPtmux;\x1b{sequence}\x1b\\"),
        None => sequence,
    };

    let mut stdout = std::io::stdout();
    stdout.write_all(sequence.as_bytes())?;
    stdout.flush()
}
