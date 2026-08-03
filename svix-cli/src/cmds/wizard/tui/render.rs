//! Drawing the quickstart: the frame's chrome and each step's body.
//!
//! Everything here reads the [`App`] state and produces lines; nothing in this file
//! mutates it. What each key does lives with the state, in the parent module.

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
    STEPS, UNLOCK_HINT,
};
use crate::{
    cmds::wizard::{
        highlight::{highlight, Syntax},
        QuickstartMode, SKILL_NAMES,
    },
    BIN_NAME,
};

impl App {
    pub(super) fn render(&self, frame: &mut Frame) {
        // The step list grows downwards as steps are reached, and the body gets the rest.
        // Nothing sits beside the body, so a wrapped URL can be selected without picking
        // up anything else on the row.
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

    /// The steps reached so far: done ones keep a check, the current one is highlighted.
    /// The ones still ahead are left out until they're reached.
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

        // A rule under the list, standing in for the panel border the body no longer has.
        lines.push(Line::styled("─".repeat(area.width as usize), DIM));
        lines.push(Line::from(""));

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn render_body(&self, frame: &mut Frame, area: Rect) {
        // The sample boxes are drawn as text, so they have to be built for this width.
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
        // Typing a token or waiting on the browser changes what the keys do, so those
        // states name their own.
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
            Step::Send if self.sent.is_none() => vec!["enter send".to_owned()],
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

    /// The tail of the agent path: the skills being installed, where they'll go, and how
    /// far through the run is.
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
        // Otherwise a run is in flight, or one failed and the error below says so. A
        // finished run doesn't get here at all: the wizard quits to hand over to the
        // agent, and so does declining the install, to print the commands instead.

        lines
    }

    /// One line per skill, marked with how far the run has got.
    ///
    /// Nothing is running until the wizard is asked to install, so until then every skill
    /// is listed as queued rather than the first one looking like it's already going.
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
            "Pick the language you'll be integrating in. The code samples in the rest of the \
             quickstart are shown in it.",
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

        let Some(qs) = &self.quickstart else {
            return lines;
        };

        lines.push(Line::from(""));
        lines.extend([field("Application", &qs.app.name), field("Id", &qs.app.id)]);
        if let Some(uid) = &qs.app.uid {
            lines.push(field("Uid", uid));
        }
        lines.push(Line::from(""));
        lines.push(prose(
            "It also got an example endpoint pointing at a Svix Play inbox, so you can preview \
             the message sent here. Your customers add their own endpoints in the \
             app portal.",
        ));
        lines.push(Line::from(""));
        lines.extend([
            field("Endpoint", &qs.endpoint.id),
            field("Delivers to", &qs.endpoint.url),
        ]);
        if let Some(channel) = &qs.channel {
            lines.push(field("Channel", channel));
        }

        lines.extend(self.code_lines(width));

        lines
    }

    /// How to send a message to the application from your own code, in the language picked
    /// on the previous step.
    fn code_lines(&self, width: u16) -> Vec<Line<'static>> {
        let language = self.language();

        let mut lines = vec![
            Line::from(""),
            Line::styled(format!("1. Install the SDK ({})", language.name), HEADING),
            Line::from(""),
        ];
        lines.extend(highlight(Syntax::Shell, language.install));
        lines.push(Line::from(""));
        lines.push(Line::styled("2. Export your API token", HEADING));
        lines.push(Line::from(""));
        lines.extend(highlight(
            Syntax::Shell,
            "export SVIX_AUTH_TOKEN='<your-token>'",
        ));
        lines.push(Line::from(""));
        lines.push(prose(
            "Create environment-specific tokens in the dashboard: https://dashboard.svix.com",
        ));
        lines.push(Line::from(""));
        lines.push(Line::styled("3. Send a message from your code", HEADING));
        lines.push(Line::from(""));
        lines.push(prose(
            "Put this where the event actually happens. The same spot you'd log it or fire an \
             internal event.",
        ));
        lines.push(Line::from(""));

        match self.sample_text() {
            Some((title, syntax, text)) => {
                lines.extend(sample(title, syntax, &text, width));
                lines.push(Line::from(""));
                lines.push(prose(
                    "In your real integration you'd create one application per customer and use \
                     that customer's application id (or uid) here, instead of the hardcoded one \
                     above.",
                ));
            }
            _ => lines.push(Line::styled(
                "Waiting for the application to be created...",
                DIM,
            )),
        }

        lines
    }

    fn send_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "This sends the exact message from the previous step, so you can see it delivered \
             before wiring up your own code.",
        )];
        lines.push(Line::from(""));

        let Some(msg) = &self.message else {
            return lines;
        };

        lines.push(field("Event type", &msg.event_type));
        lines.push(Line::from(""));
        if let Some((title, syntax, text)) = self.sample_text() {
            lines.extend(sample(title, syntax, &text, width));
        }
        lines.push(Line::from(""));

        match (&self.sent, &self.quickstart) {
            (Some(id), Some(qs)) => {
                lines.push(field("Sent", id));
                lines.push(Line::from(""));
                lines.push(prose(
                    "It was delivered to the example endpoint. Press o to open the Svix Play \
                     inbox and see the request Svix made:",
                ));
                lines.push(Line::styled(qs.play_view_url.clone(), VALUE));
                lines.push(Line::from(""));
                lines.push(prose(&format!(
                    "Every attempt is recorded: `{BIN_NAME} message-attempt list-by-msg {} {id}`",
                    qs.app.id
                )));
            }
            (None, Some(_)) => {
                lines.push(Line::styled("Press enter to send it.", HEADING));
                lines.push(Line::styled(UNLOCK_HINT, DIM));
            }
            _ => {}
        }

        lines
    }

    fn portal_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = vec![prose(
            "In production, your webhook consumers add their own endpoints inside your product \
             using the pre-built, embeddable app portal. The endpoint and message from the \
             previous steps are already in there.",
        )];
        lines.push(Line::from(""));

        match &self.portal {
            Some(url) => {
                lines.push(prose("Press o to open this one-time link:"));
                lines.push(Line::styled(url.clone(), VALUE));
            }
            None => lines.push(Line::styled("No link yet. Press r to generate one.", DIM)),
        }

        lines.push(Line::from(""));
        lines.push(prose(
            "Links like that are short-lived, so you mint them on demand from your backend and \
             link to them from your own dashboard:",
        ));
        lines.push(Line::from(""));

        if let Some((title, syntax, text)) = self.sample_text() {
            lines.extend(sample(title, syntax, &text, width));
        }
        if let Some(qs) = &self.quickstart {
            lines.push(Line::from(""));
            lines.push(prose(&format!(
                "Or from the CLI: `{BIN_NAME} authentication app-portal-access {}`",
                qs.app.id
            )));
        }

        lines
    }

    fn done_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![prose("That's the quickstart. From here you can:")];
        lines.push(Line::from(""));
        lines.extend([
            Line::styled(
                format!("  Add more event types:  {BIN_NAME} event-type create {{...}}"),
                VALUE,
            ),
            Line::styled(
                format!(
                    "  Forward webhooks locally:  {BIN_NAME} listen http://localhost:8000/webhook"
                ),
                VALUE,
            ),
            Line::styled("  Read the docs:  https://docs.svix.com", VALUE),
        ]);
        lines.push(Line::from(""));
        lines.push(prose(
            "Press q to leave. The ids and links from these steps get printed on the way out.",
        ));

        lines
    }
}
