//! Drawing the quickstart. Everything here reads the [`App`] state; nothing mutates it.

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::{
    widgets::{choices, field, prose, sample, DIM, HEADING, VALUE},
    App, Auth, Step, AUTH_CHOICES, INSTALL_CHOICES, LANGUAGE_NAMES, MODE_CHOICES, SCOPE_CHOICES,
    STEPS,
};
use crate::cmds::wizard::{highlight::Syntax, QuickstartMode, SKILL_NAMES};

impl App {
    pub(super) fn render(&self, frame: &mut Frame) {
        // The step list grows downwards as steps are reached; the body gets the rest.
        // Nothing sits beside the body, so a wrapped URL can be selected cleanly.
        let steps_height = (self.step + 3) as u16;
        let [title, steps, body, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(steps_height),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        self.render_title(frame, title);
        self.render_steps(frame, steps);
        self.render_body(frame, body);
        self.render_footer(frame, footer);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Line::styled("Svix quickstart", HEADING), area);
    }

    /// The steps reached so far; the ones still ahead are left out until they're reached.
    fn render_steps(&self, frame: &mut Frame, area: Rect) {
        let mut lines: Vec<Line> = STEPS
            .iter()
            .enumerate()
            .take(self.step + 1)
            .map(|(index, step)| {
                let (marker, style) = if self.done(index) {
                    ("✓", VALUE)
                } else if index == self.step {
                    ("▸", HEADING)
                } else {
                    ("·", DIM)
                };

                Line::styled(format!("{marker} {}", step.title()), style)
            })
            .collect();

        // A rule standing in for the panel border the body no longer has.
        lines.push(Line::styled("─".repeat(area.width as usize), DIM));
        lines.push(Line::from(""));

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn render_body(&self, frame: &mut Frame, area: Rect) {
        // The sample boxes are drawn as text, so they're built for this width.
        let width = area.width;
        self.width.set(width);
        let mut lines = self.step_lines(width);

        if let Some(busy) = self.busy {
            lines.push(Line::from(""));
            // Bold rather than yellow, which washes out on a light background.
            lines.push(Line::styled(
                busy,
                Style::new().add_modifier(Modifier::BOLD),
            ));
        }
        if let Some(status) = &self.status {
            lines.push(Line::from(""));
            lines.push(Line::styled(status.clone(), VALUE));
        }
        if let Some(error) = &self.error {
            lines.push(Line::from(""));
            lines.push(Line::styled(
                format!("Error: {error}"),
                Style::new().fg(Color::Red),
            ));
            lines.push(Line::styled("Press r to try again.", DIM));
        }

        // No border and no padding, so selecting a wrapped URL picks up the URL alone.
        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll, 0));

        frame.render_widget(paragraph, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        // Typing a token or waiting on the browser changes what the keys do.
        let special = match (self.step(), &self.auth) {
            (Step::Auth, Auth::Typing { .. }) => {
                Some("type your token  ·  enter save  ·  esc back")
            }
            (Step::Auth, Auth::Waiting { .. }) => Some("waiting for the browser  ·  q quit"),
            _ => None,
        };
        if let Some(keys) = special {
            frame.render_widget(Line::styled(keys, DIM), area);
            return;
        }

        let mut keys = match self.step() {
            Step::Application if self.quickstart.is_none() => {
                vec!["enter create the app".to_owned()]
            }
            Step::Send if self.sent.is_none() => vec!["enter send".to_owned()],
            Step::Portal if self.portal.is_none() => {
                vec!["enter generate the link".to_owned()]
            }
            Step::Done => vec!["enter finish".to_owned()],
            _ if self.choices().is_some() => vec!["enter select".to_owned()],
            _ => vec!["enter continue".to_owned()],
        };

        if self.choices().is_some() {
            keys.push("↑↓/j/k choose".to_owned());
        }
        if self.open_url().is_some() {
            keys.push("o open".to_owned());
        }
        if let Some((label, _)) = self.copy_target() {
            keys.push(format!("c copy {label}"));
        }
        if self.choices().is_none() {
            keys.push("↑↓/j/k scroll".to_owned());
        }
        keys.push("q quit".to_owned());

        frame.render_widget(Line::styled(keys.join("  ·  "), DIM), area);
    }

    pub(super) fn step_lines(&self, width: u16) -> Vec<Line<'static>> {
        match self.step() {
            Step::Auth => self.auth_lines(),
            Step::Mode => self.mode_lines(),
            Step::Language => self.language_lines(),
            Step::Application => self.application_lines(width),
            Step::Send => self.send_lines(width),
            Step::Portal => self.portal_lines(width),
            Step::Done => self.done_lines(),
        }
    }

    fn auth_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "The quickstart works against your Svix account, so it needs an API token.",
        )];
        lines.push(Line::from(""));

        match &self.auth {
            Auth::Choosing { selected } => lines.extend(choices(AUTH_CHOICES, *selected)),
            Auth::Typing { token } => {
                lines.push(prose("Paste your auth token and press enter:"));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("> ", DIM),
                    Span::styled(token.clone(), VALUE),
                    Span::styled("_", DIM),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::styled("esc to go back to the other options", DIM));
            }
            Auth::Waiting { session, .. } => {
                lines.push(prose(
                    "Approve the login in your browser, then come back here.",
                ));
                lines.push(Line::from(""));
                lines.push(field("Verification code", &session.code));
                lines.push(Line::from(""));
                lines.push(prose("If the browser didn't open, use this URL:"));
                lines.push(Line::styled(session.url.clone(), VALUE));
                lines.push(Line::from(""));
                lines.push(Line::styled("Waiting for approval...", DIM));
            }
            Auth::Done => {
                lines.push(Line::styled("You're authenticated with Svix.", VALUE));
                if let Some(url) = self.cfg.server_url() {
                    lines.push(Line::from(""));
                    lines.push(field("Server", url));
                }
                lines.push(Line::from(""));
                lines.push(Line::styled("Press enter to continue.", HEADING));
            }
        }

        lines
    }

    fn mode_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "You can walk through the rest of the quickstart yourself, or hand it to a coding \
             agent: picking the agent installs the Svix skills and hands over to them.",
        )];
        lines.push(Line::from(""));

        match self.mode {
            Some(QuickstartMode::Manual) => {
                lines.push(Line::styled("Continuing manually.", VALUE));
            }
            Some(QuickstartMode::Agent) => {
                lines.push(Line::styled("An agent will take it from here.", VALUE));
                lines.push(Line::from(""));
                lines.extend(self.skills_lines());
            }
            None => lines.extend(choices(MODE_CHOICES, self.mode_selected)),
        }

        lines
    }

    /// The agent path's tail: the skills, its two questions, and the install's progress.
    fn skills_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "The skills are installed for every coding agent found on this machine:",
        )];
        lines.push(Line::from(""));
        lines.extend(self.install_lines());
        lines.push(Line::from(""));

        if self.scope.is_none() {
            lines.push(prose("Where should they go?"));
            lines.push(Line::from(""));
            lines.extend(choices(SCOPE_CHOICES, self.scope_selected));
        } else if self.auto_install.is_none() {
            lines.push(prose("How do you want them installed?"));
            lines.push(Line::from(""));
            lines.extend(choices(INSTALL_CHOICES, self.install_selected));
        }
        // Otherwise a run is in flight or failed; anything further quits the wizard.

        lines
    }

    /// One line per skill. All are listed as queued until an install actually runs, so
    /// the first one doesn't look like it's already going.
    fn install_lines(&self) -> Vec<Line<'static>> {
        SKILL_NAMES
            .iter()
            .enumerate()
            .map(|(index, skill)| {
                let running =
                    self.scope.is_some() && self.busy.is_some() && index == self.skills_done;
                let (marker, style) = if index < self.skills_done {
                    ("✓", VALUE)
                } else if running {
                    ("▸", HEADING)
                } else {
                    ("·", DIM)
                };

                Line::styled(format!("{marker} Installing {skill} using npx"), style)
            })
            .collect()
    }

    fn language_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "Pick the language you'll be integrating in; the code samples are shown in it.",
        )];
        lines.push(Line::from(""));
        lines.extend(choices(&LANGUAGE_NAMES, self.language));

        lines
    }

    fn application_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "A consumer application defines where your messages are sent. Usually you'll want \
             one application for each of your customers.",
        )];
        lines.push(Line::from(""));

        let Some(qs) = &self.quickstart else {
            lines.push(prose("This is how you'd create one from your code:"));
            lines.push(Line::from(""));
            let language = self.language();
            if language.syntax != Syntax::Shell {
                lines.push(Line::from(format!(
                    "Install the Svix library first: {}",
                    language.install
                )));
                lines.push(Line::from(""));
            }
            if let Some((title, syntax, text)) = self.sample_text() {
                lines.extend(sample(title, syntax, &text, width));
            }
            lines.push(Line::from(""));
            lines.push(Line::styled(
                "Press enter to create the sample application.",
                HEADING,
            ));
            return lines;
        };

        lines.extend([field("Application", &qs.app.name), field("Id", &qs.app.id)]);
        if let Some(uid) = &qs.app.uid {
            lines.push(field("Uid", uid));
        }
        lines.extend([
            field("Endpoint", &qs.endpoint.id),
            field("Delivers to", &qs.endpoint.url),
        ]);
        if let Some(channel) = &qs.channel {
            lines.push(field("Channel", channel));
        }

        lines
    }

    /// How to send a message from your own code, in the language picked earlier.
    fn send_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "Now that you have an application, let's try sending a message. This uses the same \
             application id from the previous step, and the message has an event type and a \
             payload.",
        )];
        lines.push(Line::from(""));

        if let Some((title, syntax, text)) = self.sample_text() {
            lines.extend(sample(title, syntax, &text, width));
        }

        if self.sent.is_none() {
            lines.push(Line::from(""));
            lines.push(Line::styled(
                "Press enter to send this message and see it delivered.",
                HEADING,
            ));
        }

        lines
    }

    fn portal_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "Your webhook consumers manage their own endpoints in the pre-built app portal. \
             The endpoint and message from the previous steps are already in there.",
        )];
        lines.push(Line::from(""));
        lines.push(prose(
            "Access links are short-lived, so you mint them on demand from your backend and \
             link to them from your own dashboard:",
        ));
        lines.push(Line::from(""));

        if let Some((title, syntax, text)) = self.sample_text() {
            lines.extend(sample(title, syntax, &text, width));
        }
        lines.push(Line::from(""));

        match &self.portal {
            Some(url) => {
                lines.push(prose("Press o to open this one-time link:"));
                lines.push(Line::styled(url.clone(), VALUE));
            }
            None => lines.push(Line::styled(
                "Press enter to generate a link for this application.",
                HEADING,
            )),
        }

        lines
    }

    fn done_lines(&self) -> Vec<Line<'static>> {
        vec![
            prose(
                "That's the quickstart. Check out the application you created in the dashboard: \
                 https://dashboard.svix.com",
            ),
            Line::from(""),
            prose("The docs at https://docs.svix.com go deeper."),
            Line::from(""),
            prose("Press q to leave."),
        ]
    }
}
