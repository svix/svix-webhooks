//! The full-screen quickstart, steps 3 to 6.
//!
//! Every step is a page you can walk back and forth through, so nothing is a point of no
//! return: the only key that changes anything on your account is Enter, on the steps that
//! say so.

use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    DefaultTerminal, Frame,
};
use svix::api::Svix;

use super::{
    create_application, portal_url, sample_message, send_message, Language, Quickstart,
    SampleMessage, DEFAULT_SERVER_URL, LANGUAGES,
};
use crate::{config::Config, BIN_NAME};

/// Runs the quickstart UI, then prints what the user will want to keep.
pub(super) async fn run(client: &Svix, cfg: &Config) -> anyhow::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let result = App::new(client, cfg).run(&mut terminal).await;
    ratatui::restore();

    // The alternate screen is gone by now, so anything worth keeping has to be reprinted.
    result?.print_summary();

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Application,
    Code,
    Send,
    Portal,
    Done,
}

const STEPS: &[Step] = &[
    Step::Application,
    Step::Code,
    Step::Send,
    Step::Portal,
    Step::Done,
];

impl Step {
    fn title(self) -> &'static str {
        match self {
            Self::Application => "3 Application",
            Self::Code => "4 Your code",
            Self::Send => "5 First message",
            Self::Portal => "6 App portal",
            Self::Done => "Done",
        }
    }
}

/// Work that talks to the API, run between frames so the UI can show it's busy.
#[derive(Clone, Copy)]
enum Action {
    Prepare,
    Send,
    Portal,
}

impl Action {
    fn label(self) -> &'static str {
        match self {
            Self::Prepare => "Creating the application and its example endpoint...",
            Self::Send => "Sending the message...",
            Self::Portal => "Generating an app portal link...",
        }
    }
}

struct App<'a> {
    client: &'a Svix,
    cfg: &'a Config,

    step: usize,
    language: usize,
    scroll: u16,
    quit: bool,

    message: Option<SampleMessage>,
    quickstart: Option<Quickstart>,
    sent: Option<String>,
    portal: Option<String>,

    pending: Option<Action>,
    busy: Option<&'static str>,
    status: Option<String>,
    error: Option<String>,
}

impl<'a> App<'a> {
    fn new(client: &'a Svix, cfg: &'a Config) -> Self {
        Self {
            client,
            cfg,
            step: 0,
            language: 0,
            scroll: 0,
            quit: false,
            message: None,
            quickstart: None,
            sent: None,
            portal: None,
            // The application is created up front: every later step refers to it.
            pending: Some(Action::Prepare),
            busy: None,
            status: None,
            error: None,
        }
    }

    async fn run(mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<Self> {
        while !self.quit {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(action) = self.pending.take() {
                self.busy = Some(action.label());
                terminal.draw(|frame| self.render(frame))?;

                let outcome = self.perform(action).await;
                self.busy = None;
                if let Err(e) = outcome {
                    self.error = Some(format!("{e:#}"));
                }
                continue;
            }

            // Short poll so a resize repaints promptly without spinning the CPU.
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.on_key(key);
                    }
                }
            }
        }

        Ok(self)
    }

    async fn perform(&mut self, action: Action) -> anyhow::Result<()> {
        match action {
            Action::Prepare => {
                // The event type has to exist before the endpoint that filters on it.
                let message = sample_message(self.client).await?;
                self.quickstart = Some(create_application(self.client, &message).await?);
                self.message = Some(message);
            }
            Action::Send => {
                let (qs, msg) = match (&self.quickstart, &self.message) {
                    (Some(qs), Some(msg)) => (qs, msg),
                    _ => return Ok(()),
                };
                let sent = send_message(self.client, qs, msg).await?;
                self.status = Some(format!("Sent message {}", sent.id));
                self.sent = Some(sent.id);
            }
            Action::Portal => {
                let Some(qs) = &self.quickstart else {
                    return Ok(());
                };
                self.portal = Some(portal_url(self.client, qs).await?);
            }
        }

        Ok(())
    }

    fn step(&self) -> Step {
        STEPS[self.step]
    }

    fn language(&self) -> &'static Language {
        &LANGUAGES[self.language]
    }

    /// The URL `o` opens on the current step, if any.
    fn open_url(&self) -> Option<&str> {
        match self.step() {
            Step::Send => self.quickstart.as_ref().map(|qs| qs.play_view_url.as_str()),
            Step::Portal => self.portal.as_deref(),
            _ => None,
        }
    }

    fn on_key(&mut self, key: KeyEvent) {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        self.status = None;

        match key.code {
            KeyCode::Char('c') if ctrl => self.quit = true,
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,

            KeyCode::Left | KeyCode::Char('h' | 'a') | KeyCode::BackTab => self.go(-1),
            KeyCode::Right | KeyCode::Char('l' | 'd') | KeyCode::Tab => self.go(1),

            // On the code step the list takes the vertical keys; elsewhere they scroll.
            KeyCode::Up | KeyCode::Char('k') if self.step() == Step::Code => {
                self.language = self.language.saturating_sub(1);
                self.scroll = 0;
            }
            KeyCode::Down | KeyCode::Char('j') if self.step() == Step::Code => {
                self.language = (self.language + 1).min(LANGUAGES.len() - 1);
                self.scroll = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
            KeyCode::PageDown => self.scroll_down(10),

            KeyCode::Enter => self.activate(),
            KeyCode::Char('o') => self.open_in_browser(),
            KeyCode::Char('r') => self.retry(),
            _ => {}
        }
    }

    /// Scrolls down, stopping at the last line so the page can't run off into blank space.
    /// Wrapping means the real height is at least this, which is close enough for a bound.
    fn scroll_down(&mut self, lines: u16) {
        let max = self.step_lines().len().saturating_sub(1) as u16;
        self.scroll = self.scroll.saturating_add(lines).min(max);
    }

    fn go(&mut self, delta: isize) {
        let next = (self.step as isize + delta).clamp(0, STEPS.len() as isize - 1) as usize;
        if next == self.step {
            return;
        }
        self.step = next;
        self.scroll = 0;

        // Each step generates what it needs the first time you reach it, except the
        // message, which only goes out when you ask for it.
        if self.step() == Step::Portal && self.portal.is_none() && self.quickstart.is_some() {
            self.pending = Some(Action::Portal);
        }
    }

    fn activate(&mut self) {
        match self.step() {
            Step::Application if self.quickstart.is_none() => self.pending = Some(Action::Prepare),
            Step::Send if self.sent.is_none() && self.quickstart.is_some() => {
                self.pending = Some(Action::Send);
            }
            Step::Send | Step::Portal => self.open_in_browser(),
            Step::Done => self.quit = true,
            _ => self.go(1),
        }
    }

    /// Re-runs whatever the current step needs, for when a call failed.
    fn retry(&mut self) {
        self.error = None;
        match self.step() {
            Step::Application if self.quickstart.is_none() => self.pending = Some(Action::Prepare),
            Step::Portal => {
                self.portal = None;
                self.pending = Some(Action::Portal);
            }
            _ => {}
        }
    }

    fn open_in_browser(&mut self) {
        let Some(url) = self.open_url().map(ToOwned::to_owned) else {
            return;
        };

        self.status = Some(match open::that(&url) {
            Ok(()) => format!("Opened {url}"),
            Err(e) => format!("Couldn't open a browser ({e}). Copy the URL above instead."),
        });
    }

    /// Reprints the ids and URLs once the alternate screen is gone.
    fn print_summary(&self) {
        let Some(qs) = &self.quickstart else {
            return;
        };

        println!("Quickstart summary:");
        println!("  Application:  {} ({})", qs.app.name, qs.app.id);
        if let Some(uid) = &qs.app.uid {
            println!("  Uid:          {uid}");
        }
        println!("  Endpoint:     {} -> {}", qs.endpoint.id, qs.endpoint.url);
        println!("  Play inbox:   {}", qs.play_view_url);
        if let Some(id) = &self.sent {
            println!("  Message sent: {id}");
        }
        if let Some(url) = &self.portal {
            println!("  App portal:   {url}");
        }

        println!("\nFrom here you can:");
        println!("  - Add more event types:  `{BIN_NAME} event-type create {{...}}`");
        println!(
            "  - Forward webhooks to a local server:  `{BIN_NAME} listen http://localhost:8000/webhook`"
        );
        println!("  - Read the docs:  https://docs.svix.com\n");
    }
}

// --- rendering -----------------------------------------------------------------------

// Prose and code keep the terminal's own foreground colour, and anything secondary is
// dimmed rather than greyed: a fixed grey is unreadable on a light background.
const HEADING: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
const VALUE: Style = Style::new().fg(Color::Green);
const DIM: Style = Style::new().add_modifier(Modifier::DIM);
const TEXT: Style = Style::new();

impl App<'_> {
    fn render(&self, frame: &mut Frame) {
        let [tabs, body, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        self.render_tabs(frame, tabs);

        if self.step() == Step::Code {
            let [list, snippet] =
                Layout::horizontal([Constraint::Length(30), Constraint::Min(0)]).areas(body);
            self.render_languages(frame, list);
            self.render_body(frame, snippet);
        } else {
            self.render_body(frame, body);
        }

        self.render_footer(frame, footer);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(STEPS.iter().map(|step| step.title()))
            .select(self.step)
            .style(DIM)
            .highlight_style(HEADING)
            .divider("·");

        frame.render_widget(tabs, area);
    }

    fn render_languages(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = LANGUAGES
            .iter()
            .map(|language| ListItem::new(language.name))
            .collect();
        let list = List::new(items)
            .block(Block::bordered().title(" Language "))
            .highlight_style(Style::new().fg(Color::Black).bg(Color::Cyan))
            .highlight_symbol("› ");

        let mut state = ListState::default().with_selected(Some(self.language));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn render_body(&self, frame: &mut Frame, area: Rect) {
        let mut lines = self.step_lines();

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

        let title = format!(" {} ", self.step().title());
        let paragraph = Paragraph::new(lines)
            .block(Block::bordered().title(title))
            .wrap(Wrap { trim: false })
            .scroll((self.scroll, 0));

        frame.render_widget(paragraph, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let mut keys = vec![
            "←/a/h prev".to_owned(),
            "→/d/l next".to_owned(),
            if self.step() == Step::Code {
                "↑↓/j/k language".to_owned()
            } else {
                "↑↓/j/k scroll".to_owned()
            },
        ];

        match self.step() {
            Step::Send if self.sent.is_none() => keys.push("enter send".to_owned()),
            Step::Send | Step::Portal => keys.push("o open".to_owned()),
            Step::Done => keys.push("enter finish".to_owned()),
            _ => {}
        }
        keys.push("q quit".to_owned());

        frame.render_widget(Line::styled(format!(" {}", keys.join("  ·  ")), DIM), area);
    }

    fn step_lines(&self) -> Vec<Line<'static>> {
        match self.step() {
            Step::Application => self.application_lines(),
            Step::Code => self.code_lines(),
            Step::Send => self.send_lines(),
            Step::Portal => self.portal_lines(),
            Step::Done => self.done_lines(),
        }
    }

    fn application_lines(&self) -> Vec<Line<'static>> {
        let mut lines = wrap_text(
            "A consumer application defines where your messages are sent. Usually you'll want \
             one application for each of your customers.",
        );

        let Some(qs) = &self.quickstart else {
            return lines;
        };

        lines.push(Line::from(""));
        lines.extend([field("Application", &qs.app.name), field("Id", &qs.app.id)]);
        if let Some(uid) = &qs.app.uid {
            lines.push(field("Uid", uid));
        }
        lines.push(Line::from(""));
        lines.extend(wrap_text(
            "It also got an example endpoint pointing at a Svix Play inbox, so the message you \
             send in step 5 has somewhere to go — your customers add their own endpoints in the \
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

        lines
    }

    fn code_lines(&self) -> Vec<Line<'static>> {
        let language = self.language();
        let server_url = self.cfg.server_url().unwrap_or(DEFAULT_SERVER_URL);

        let mut lines = vec![Line::styled("1. Install the SDK", HEADING), Line::from("")];
        lines.extend(code(language.install));
        lines.push(Line::from(""));
        lines.push(Line::styled("2. Export your API token", HEADING));
        lines.push(Line::from(""));
        lines.extend(code("export SVIX_AUTH_TOKEN='<your-token>'"));
        lines.push(Line::from(""));
        lines.extend(wrap_text(
            "Create environment-specific tokens in the dashboard: https://dashboard.svix.com",
        ));
        lines.push(Line::from(""));
        lines.push(Line::styled("3. Send a message from your code", HEADING));
        lines.push(Line::from(""));
        lines.extend(wrap_text(
            "Put this where the event actually happens — the same spot you'd log it or fire an \
             internal event.",
        ));
        lines.push(Line::from(""));

        match (&self.quickstart, &self.message) {
            (Some(qs), Some(msg)) => {
                lines.extend(code(&(language.snippet)(&qs.app.id, msg, server_url)));
                lines.push(Line::from(""));
                lines.extend(wrap_text(
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

    fn send_lines(&self) -> Vec<Line<'static>> {
        let mut lines = wrap_text(
            "This sends the exact message from the previous step, so you can see it delivered \
             before wiring up your own code.",
        );
        lines.push(Line::from(""));

        let Some(msg) = &self.message else {
            return lines;
        };

        lines.push(field("Event type", &msg.event_type));
        lines.push(Line::from(""));
        lines.extend(code(
            &serde_json::to_string_pretty(&msg.payload).unwrap_or_default(),
        ));
        lines.push(Line::from(""));

        match (&self.sent, &self.quickstart) {
            (Some(id), Some(qs)) => {
                lines.push(field("Sent", id));
                lines.push(Line::from(""));
                lines.extend(wrap_text(
                    "It was delivered to the example endpoint. Press o to open the Svix Play \
                     inbox and see the request Svix made:",
                ));
                lines.push(Line::styled(qs.play_view_url.clone(), VALUE));
                lines.push(Line::from(""));
                lines.extend(wrap_text(&format!(
                    "Every attempt is recorded: `{BIN_NAME} message-attempt list-by-msg {} {id}`",
                    qs.app.id
                )));
            }
            (None, Some(_)) => lines.push(Line::styled("Press enter to send it.", HEADING)),
            _ => {}
        }

        lines
    }

    fn portal_lines(&self) -> Vec<Line<'static>> {
        let mut lines = wrap_text(
            "In production, your webhook consumers add their own endpoints inside your product \
             using the pre-built, embeddable app portal. The endpoint and message from the \
             previous steps are already in there.",
        );
        lines.push(Line::from(""));

        match &self.portal {
            Some(url) => {
                lines.extend(wrap_text("Press o to open this one-time link:"));
                lines.push(Line::styled(url.clone(), VALUE));
            }
            None => lines.push(Line::styled("No link yet — press r to generate one.", DIM)),
        }

        lines.push(Line::from(""));
        lines.extend(wrap_text(
            "Links like that are short-lived, so you mint them on demand from your backend and \
             link to them from your own dashboard:",
        ));
        lines.push(Line::from(""));

        if let Some(qs) = &self.quickstart {
            let server_url = self.cfg.server_url().unwrap_or(DEFAULT_SERVER_URL);
            lines.extend(code(&(self.language().portal)(&qs.app.id, server_url)));
            lines.push(Line::from(""));
            lines.extend(wrap_text(&format!(
                "Or from the CLI: `{BIN_NAME} authentication app-portal-access {}`",
                qs.app.id
            )));
        }

        lines
    }

    fn done_lines(&self) -> Vec<Line<'static>> {
        let mut lines = wrap_text("That's the quickstart. From here you can:");
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
        lines.extend(wrap_text(
            "Press q to leave — the ids and links from these steps get printed on the way out.",
        ));

        lines
    }
}

/// A `label: value` line, with the value in the colour used for things worth copying.
fn field(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label}: "), DIM),
        Span::styled(value.to_owned(), VALUE),
    ])
}

/// Code samples, kept on their own lines so the paragraph widget doesn't reflow them.
fn code(text: &str) -> Vec<Line<'static>> {
    text.lines()
        .map(|line| Line::styled(line.to_owned(), TEXT))
        .collect()
}

/// Prose is wrapped by the paragraph widget, so this just owns the text.
fn wrap_text(text: &str) -> Vec<Line<'static>> {
    vec![Line::styled(text.to_owned(), TEXT)]
}

#[cfg(test)]
mod tests {
    use ratatui::{backend::TestBackend, Terminal};
    use svix::api::Svix;

    use super::*;

    /// Renders every step at a small size, which is where layout and scroll bugs show up.
    #[test]
    fn every_step_renders() {
        let client = Svix::new("testsk_fake".to_owned(), None);
        let cfg = Config::default();
        let mut app = App::new(&client, &cfg);
        // Nothing has run yet, so this is also the "still loading" state of each step.
        app.pending = None;

        let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("terminal");

        for step in 0..STEPS.len() {
            app.step = step;
            terminal.draw(|frame| app.render(frame)).expect("draw");

            let rendered: String = terminal
                .backend()
                .buffer()
                .content()
                .iter()
                .map(|cell| cell.symbol())
                .collect();

            assert!(
                rendered.contains("prev"),
                "the footer should always show how to navigate"
            );
        }
    }

    #[test]
    fn arrows_wasd_and_vim_keys_all_move_between_steps() {
        let client = Svix::new("testsk_fake".to_owned(), None);
        let cfg = Config::default();
        let mut app = App::new(&client, &cfg);
        app.pending = None;

        for forward in [KeyCode::Right, KeyCode::Char('d'), KeyCode::Char('l')] {
            app.step = 0;
            app.on_key(KeyEvent::from(forward));
            assert_eq!(app.step, 1, "{forward:?} should move forward");
        }

        for back in [KeyCode::Left, KeyCode::Char('a'), KeyCode::Char('h')] {
            app.step = 1;
            app.on_key(KeyEvent::from(back));
            assert_eq!(app.step, 0, "{back:?} should move back");
        }

        // The ends of the wizard don't wrap around.
        app.step = 0;
        app.on_key(KeyEvent::from(KeyCode::Left));
        assert_eq!(app.step, 0);
        app.step = STEPS.len() - 1;
        app.on_key(KeyEvent::from(KeyCode::Right));
        assert_eq!(app.step, STEPS.len() - 1);
    }

    #[test]
    fn vertical_keys_pick_a_language_on_the_code_step_and_scroll_elsewhere() {
        let client = Svix::new("testsk_fake".to_owned(), None);
        let cfg = Config::default();
        let mut app = App::new(&client, &cfg);
        app.pending = None;

        app.step = STEPS
            .iter()
            .position(|s| *s == Step::Code)
            .expect("code step");
        app.on_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(app.language, 1);
        assert_eq!(app.scroll, 0);

        app.step = STEPS
            .iter()
            .position(|s| *s == Step::Send)
            .expect("send step");
        app.on_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(
            app.language, 1,
            "the language shouldn't change off the code step"
        );
        assert_eq!(app.scroll, 1);
    }
}
