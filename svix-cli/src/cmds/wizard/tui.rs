//! The full-screen quickstart.
//!
//! The steps are a checklist you work down: each one builds on the one before it, so the
//! wizard only moves forward, ticking steps off as they're done. The only key that
//! changes anything on your account is Enter.

use std::{
    cell::Cell,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use svix::api::Svix;

use super::{
    create_application,
    highlight::{background, highlight, Syntax},
    install_skill, portal_url, sample_message, send_message, Language, Quickstart, QuickstartMode,
    SampleMessage, SkillScope, DEFAULT_SERVER_URL, LANGUAGES, SKILL_NAMES,
};
use crate::{
    cmds::login::{self, DashboardLogin},
    config::Config,
    BIN_NAME,
};

/// Runs the quickstart UI, then prints what the user will want to keep.
///
/// Returns how the user chose to continue. The agent path only counts as taken once its
/// skills are installed, so quitting part way through doesn't hand over to an agent that
/// has nothing to work with.
pub(super) async fn run() -> anyhow::Result<QuickstartMode> {
    let mut terminal = ratatui::try_init()?;
    let result = App::new().run(&mut terminal).await;
    ratatui::restore();

    let app = result?;
    // The alternate screen is gone by now, so anything worth keeping has to be reprinted.
    app.print_summary();

    Ok(match app.mode {
        Some(QuickstartMode::Agent) if app.skills_installed => QuickstartMode::Agent,
        _ => QuickstartMode::Manual,
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Auth,
    Mode,
    Language,
    Application,
    Send,
    Portal,
    Done,
}

const STEPS: &[Step] = &[
    Step::Auth,
    Step::Mode,
    Step::Language,
    Step::Application,
    Step::Send,
    Step::Portal,
    Step::Done,
];

impl Step {
    fn title(self) -> &'static str {
        match self {
            Self::Auth => "Authenticate",
            Self::Mode => "How to continue",
            Self::Language => "Language",
            Self::Application => "Application",
            Self::Send => "First message",
            Self::Portal => "App portal",
            Self::Done => "Done",
        }
    }
}

/// Shown on the message step, and when the next step is reached for before it's ready.
const UNLOCK_HINT: &str = "The next step will be unlocked after you send the message.";

/// Work that talks to the API, run between frames so the UI can show it's busy.
#[derive(Clone, Copy)]
enum Action {
    StartLogin,
    SaveToken,
    Prepare,
    Send,
    Portal,
    InstallSkills,
}

impl Action {
    fn label(self) -> &'static str {
        match self {
            Self::StartLogin => "Starting the login...",
            Self::SaveToken => "Saving your token...",
            Self::Prepare => "Creating the application and its example endpoint...",
            Self::Send => "Sending the message...",
            Self::Portal => "Generating an app portal link...",
            Self::InstallSkills => "Running the install...",
        }
    }
}

/// How far along the login step is.
enum Auth {
    /// Picking between logging in through the dashboard and pasting a token.
    Choosing { selected: usize },
    /// Typing a token in by hand.
    Typing { token: String },
    /// Waiting for the login to be approved in the browser.
    Waiting {
        session: Box<DashboardLogin>,
        started: Instant,
        last_poll: Instant,
    },
    /// There's a usable token in the config.
    Done,
}

const AUTH_CHOICES: &[&str] = &["Log in through dashboard.svix.com", "Paste an auth token"];
const MODE_CHOICES: &[&str] = &[
    "Continue manually (I'll walk through the steps myself)",
    "Continue with an agent (let a coding agent set things up)",
];
const SCOPE_CHOICES: &[&str] = &[
    "This project only (./.agents/skills)",
    "Globally, for every project (~/.agents/skills)",
];

struct App {
    cfg: Config,
    client: Option<Svix>,

    step: usize,
    language: usize,
    scroll: u16,
    quit: bool,
    /// Width of the body area at the last render, used to size the sample boxes.
    width: Cell<u16>,

    auth: Auth,
    mode: Option<QuickstartMode>,
    mode_selected: usize,
    /// Set on the agent path, once the user has picked where the skills go.
    scope: Option<SkillScope>,
    scope_selected: usize,
    skills_installed: bool,
    /// How many of `SKILL_NAMES` are installed, which is also the index of the command
    /// currently running while the install is in flight.
    skills_done: usize,

    message: Option<SampleMessage>,
    quickstart: Option<Quickstart>,
    sent: Option<String>,
    portal: Option<String>,

    pending: Option<Action>,
    busy: Option<&'static str>,
    status: Option<String>,
    error: Option<String>,
}

impl App {
    fn new() -> Self {
        // A token in the config (or the environment) means the login step is already done.
        let cfg = Config::load().unwrap_or_default();
        let authenticated = cfg
            .auth_token
            .as_ref()
            .is_some_and(|token| !token.trim().is_empty());

        Self {
            client: authenticated
                .then(|| crate::get_client(&cfg).ok())
                .flatten(),
            cfg,
            step: 0,
            language: 0,
            scroll: 0,
            quit: false,
            width: Cell::new(80),
            auth: if authenticated {
                Auth::Done
            } else {
                Auth::Choosing { selected: 0 }
            },
            mode: None,
            mode_selected: 0,
            scope: None,
            scope_selected: 0,
            skills_installed: false,
            skills_done: 0,
            message: None,
            quickstart: None,
            sent: None,
            portal: None,
            pending: None,
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

                let outcome = self.perform(action, terminal).await;
                self.busy = None;
                if let Err(e) = outcome {
                    self.error = Some(format!("{e:#}"));
                }
                continue;
            }

            if let Err(e) = self.poll_login().await {
                self.error = Some(format!("{e:#}"));
                self.auth = Auth::Choosing { selected: 0 };
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

    /// The terminal comes along so a multi-command action can repaint between them.
    async fn perform(
        &mut self,
        action: Action,
        terminal: &mut DefaultTerminal,
    ) -> anyhow::Result<()> {
        match action {
            Action::StartLogin => {
                let session = login::DashboardLogin::start().await?;
                let opened = open::that(&session.url).is_ok();
                if !opened {
                    self.status = Some("Couldn't open a browser. Copy the URL above.".to_owned());
                }
                let now = Instant::now();
                self.auth = Auth::Waiting {
                    session: Box::new(session),
                    started: now,
                    last_poll: now,
                };
            }
            Action::SaveToken => {
                let Auth::Typing { token } = &self.auth else {
                    return Ok(());
                };
                self.finish_login(token.trim().to_owned())?;
            }
            Action::Prepare => {
                let Some(client) = self.client.clone() else {
                    return Ok(());
                };
                // The event type has to exist before the endpoint that filters on it.
                let message = sample_message(&client).await?;
                self.quickstart = Some(create_application(&client, &message).await?);
                self.message = Some(message);
            }
            Action::Send => {
                let (Some(client), Some(qs), Some(msg)) =
                    (self.client.clone(), &self.quickstart, &self.message)
                else {
                    return Ok(());
                };
                let sent = send_message(&client, qs, msg).await?;
                self.status = Some(format!("Sent message {}", sent.id));
                self.sent = Some(sent.id);
            }
            Action::Portal => {
                let (Some(client), Some(qs)) = (self.client.clone(), &self.quickstart) else {
                    return Ok(());
                };
                self.portal = Some(portal_url(&client, qs).await?);
            }
            Action::InstallSkills => {
                let Some(scope) = self.scope else {
                    return Ok(());
                };

                self.skills_done = 0;
                for skill in SKILL_NAMES {
                    // Each command is marked as running before it starts, so the screen
                    // shows which one the wizard is sitting on.
                    terminal.draw(|frame| self.render(frame))?;
                    // `npx` is slow enough to be worth getting off the UI thread, and the
                    // wizard has nothing else to do until it's finished.
                    tokio::task::spawn_blocking(move || install_skill(skill, scope)).await??;
                    self.skills_done += 1;
                }

                self.skills_installed = true;
                // The agent takes it from here, so there's nothing left to show.
                self.quit = true;
            }
        }

        Ok(())
    }

    /// Makes one login polling attempt per interval, so the UI keeps drawing while the
    /// user is off approving the login in their browser.
    async fn poll_login(&mut self) -> anyhow::Result<()> {
        let Auth::Waiting {
            session,
            started,
            last_poll,
        } = &mut self.auth
        else {
            return Ok(());
        };

        if started.elapsed() > login::MAX_POLL_TIME {
            self.auth = Auth::Choosing { selected: 0 };
            anyhow::bail!("The login expired before it was approved.");
        }
        if last_poll.elapsed() < login::POLL_INTERVAL {
            return Ok(());
        }

        *last_poll = Instant::now();
        if let Some(token) = session.poll().await? {
            self.finish_login(token)?;
        }

        Ok(())
    }

    /// Saves the token, builds the client from it, and ticks the login step off.
    fn finish_login(&mut self, token: String) -> anyhow::Result<()> {
        if token.is_empty() {
            self.status = Some("That token was empty.".to_owned());
            return Ok(());
        }

        login::save_auth_token(token)?;
        self.cfg = Config::load()?;
        self.client = Some(crate::get_client(&self.cfg)?);
        self.auth = Auth::Done;
        self.status = Some("You're authenticated with Svix.".to_owned());

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
            Step::Send if self.sent.is_some() => {
                self.quickstart.as_ref().map(|qs| qs.play_view_url.as_str())
            }
            Step::Portal => self.portal.as_deref(),
            _ => None,
        }
    }

    fn on_key(&mut self, key: KeyEvent) {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        self.status = None;

        // Typing a token takes over the keyboard: every printable key is part of it.
        if let (Step::Auth, Auth::Typing { token }) = (self.step(), &mut self.auth) {
            match key.code {
                KeyCode::Char('c') if ctrl => self.quit = true,
                KeyCode::Char(c) => token.push(c),
                KeyCode::Backspace => {
                    token.pop();
                }
                KeyCode::Esc => self.auth = Auth::Choosing { selected: 1 },
                KeyCode::Enter => self.pending = Some(Action::SaveToken),
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('c') if ctrl => self.quit = true,
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,

            // The quickstart only moves forward, so the horizontal keys do nothing.
            KeyCode::Left
            | KeyCode::Right
            | KeyCode::Char('h' | 'l')
            | KeyCode::Tab
            | KeyCode::BackTab => {}

            // Steps that offer a choice put the vertical keys on it; the rest scroll.
            KeyCode::Up | KeyCode::Char('k') if self.choices().is_some() => self.select(-1),
            KeyCode::Down | KeyCode::Char('j') if self.choices().is_some() => self.select(1),
            KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
            KeyCode::PageDown => self.scroll_down(10),

            KeyCode::Enter => self.activate(),
            KeyCode::Char('o') => self.open_in_browser(),
            KeyCode::Char('c') => self.copy(),
            KeyCode::Char('r') => self.retry(),
            _ => {}
        }
    }

    /// Scrolls down, stopping at the last line so the page can't run off into blank space.
    /// Wrapping means the real height is at least this, which is close enough for a bound.
    fn scroll_down(&mut self, lines: u16) {
        let max = self.step_lines(self.width.get()).len().saturating_sub(1) as u16;
        self.scroll = self.scroll.saturating_add(lines).min(max);
    }

    /// The options the current step is asking the user to pick from, if it is.
    fn choices(&self) -> Option<(Vec<&'static str>, usize)> {
        match (self.step(), &self.auth) {
            (Step::Auth, Auth::Choosing { selected }) => Some((AUTH_CHOICES.to_vec(), *selected)),
            (Step::Mode, _) if self.mode.is_none() => {
                Some((MODE_CHOICES.to_vec(), self.mode_selected))
            }
            // The agent path stays on this step to ask where the skills should go.
            (Step::Mode, _) if self.mode == Some(QuickstartMode::Agent) && self.scope.is_none() => {
                Some((SCOPE_CHOICES.to_vec(), self.scope_selected))
            }
            (Step::Language, _) => Some((
                LANGUAGES.iter().map(|language| language.name).collect(),
                self.language,
            )),
            _ => None,
        }
    }

    fn select(&mut self, delta: isize) {
        let Some((options, selected)) = self.choices() else {
            return;
        };
        let next = (selected as isize + delta).clamp(0, options.len() as isize - 1) as usize;

        match self.step() {
            Step::Auth => self.auth = Auth::Choosing { selected: next },
            Step::Language => self.language = next,
            Step::Mode if self.mode.is_some() => self.scope_selected = next,
            _ => self.mode_selected = next,
        }
    }

    /// Whether a step is done, which is what puts a check next to it in the list.
    fn done(&self, index: usize) -> bool {
        match STEPS[index] {
            Step::Auth => matches!(self.auth, Auth::Done),
            Step::Mode => self.mode.is_some(),
            // The language is picked by moving on from the step, so it counts once you have.
            Step::Language => index < self.step,
            Step::Application => self.quickstart.is_some(),
            Step::Send => self.sent.is_some(),
            Step::Portal => self.portal.is_some(),
            Step::Done => false,
        }
    }

    /// What the current step still needs before the quickstart can move on, if anything.
    fn blocker(&self) -> Option<&'static str> {
        match self.step() {
            Step::Auth if !matches!(self.auth, Auth::Done) => Some("Log in before continuing."),
            Step::Application if self.quickstart.is_none() => {
                Some("The application is still being created.")
            }
            Step::Send if self.sent.is_none() => Some(UNLOCK_HINT),
            _ => None,
        }
    }

    /// Enter does the current step's work, or moves on once it's done. There's no way
    /// back: each step builds on the one before it.
    fn activate(&mut self) {
        match self.step() {
            // The login step's Enter starts whichever way of logging in was picked.
            Step::Auth => match &self.auth {
                Auth::Choosing { selected: 0 } => {
                    self.pending = Some(Action::StartLogin);
                    return;
                }
                Auth::Choosing { .. } => {
                    self.auth = Auth::Typing {
                        token: String::new(),
                    };
                    return;
                }
                Auth::Waiting { .. } | Auth::Typing { .. } => return,
                Auth::Done => {}
            },
            Step::Mode if self.mode.is_none() => {
                let mode = if self.mode_selected == 0 {
                    QuickstartMode::Manual
                } else {
                    QuickstartMode::Agent
                };
                self.mode = Some(mode);

                // The agent path stays here to ask where the skills should go.
                if mode == QuickstartMode::Agent {
                    return;
                }
            }
            // The rest of the agent path: pick where the skills go, then install them.
            // A failed install stays here rather than falling through to the manual steps,
            // so Enter is another go at it.
            Step::Mode if self.mode == Some(QuickstartMode::Agent) => {
                if self.scope.is_none() {
                    self.scope = Some(if self.scope_selected == 0 {
                        SkillScope::Project
                    } else {
                        SkillScope::Global
                    });
                }
                self.error = None;
                self.pending = Some(Action::InstallSkills);
                return;
            }
            Step::Send if self.sent.is_none() && self.quickstart.is_some() => {
                self.pending = Some(Action::Send);
                return;
            }
            Step::Done => {
                self.quit = true;
                return;
            }
            _ => {}
        }

        if let Some(blocker) = self.blocker() {
            self.status = Some(blocker.to_owned());
            return;
        }

        self.step = (self.step + 1).min(STEPS.len() - 1);
        self.scroll = 0;

        // Each step generates what it needs when you reach it, except the message, which
        // only goes out when you ask for it.
        match self.step() {
            Step::Application if self.quickstart.is_none() && self.client.is_some() => {
                self.pending = Some(Action::Prepare);
            }
            Step::Portal if self.portal.is_none() && self.quickstart.is_some() => {
                self.pending = Some(Action::Portal);
            }
            _ => {}
        }
    }

    /// Re-runs whatever the current step needs, for when a call failed.
    fn retry(&mut self) {
        self.error = None;
        match self.step() {
            Step::Application if self.quickstart.is_none() => self.pending = Some(Action::Prepare),
            // A failed install leaves the scope picked, so retrying goes straight to `npx`.
            Step::Mode if self.scope.is_some() && !self.skills_installed => {
                self.pending = Some(Action::InstallSkills);
            }
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

impl App {
    fn render(&self, frame: &mut Frame) {
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
        // Typing a token swallows the usual keys, so it gets its own footer.
        if matches!((self.step(), &self.auth), (Step::Auth, Auth::Typing { .. })) {
            let keys = "type your token  ·  enter save  ·  esc back";
            frame.render_widget(Line::styled(keys, DIM), area);
            return;
        }

        if matches!(
            (self.step(), &self.auth),
            (Step::Auth, Auth::Waiting { .. })
        ) {
            let keys = "waiting for the browser  ·  q quit";
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

    /// The code sample the current step shows, as its title and text.
    fn sample_text(&self) -> Option<(&'static str, Syntax, String)> {
        let server_url = self.cfg.server_url().unwrap_or(DEFAULT_SERVER_URL);
        let language = self.language();

        match self.step() {
            Step::Application => {
                let (qs, msg) = (self.quickstart.as_ref()?, self.message.as_ref()?);
                Some((
                    language.name,
                    language.syntax,
                    (language.snippet)(&qs.app.id, msg, server_url),
                ))
            }
            Step::Send => {
                let msg = self.message.as_ref()?;
                Some((
                    "Payload",
                    Syntax::Json,
                    serde_json::to_string_pretty(&msg.payload).ok()?,
                ))
            }
            Step::Portal => {
                let qs = self.quickstart.as_ref()?;
                Some((
                    language.name,
                    language.syntax,
                    (language.portal)(&qs.app.id, server_url),
                ))
            }
            _ => None,
        }
    }

    /// What `c` copies: the step's link if it has one, otherwise its code sample.
    fn copy_target(&self) -> Option<(&'static str, String)> {
        if let Some(url) = self.open_url() {
            return Some(("link", url.to_owned()));
        }
        let (_, _, text) = self.sample_text()?;
        Some(("sample", text))
    }

    fn copy(&mut self) {
        let Some((label, text)) = self.copy_target() else {
            return;
        };

        self.status = Some(match copy_to_clipboard(&text) {
            Ok(()) => format!("Copied the {label} to your clipboard."),
            Err(e) => format!("Couldn't copy the {label} ({e})."),
        });
    }

    fn step_lines(&self, width: u16) -> Vec<Line<'static>> {
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
        let mut lines =
            wrap_text("The quickstart works against your Svix account, so it needs an API token.");
        lines.push(Line::from(""));

        match &self.auth {
            Auth::Choosing { selected } => lines.extend(choices(AUTH_CHOICES, *selected)),
            Auth::Typing { token } => {
                lines.extend(wrap_text("Paste your auth token and press enter:"));
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
                lines.extend(wrap_text(
                    "Approve the login in your browser, then come back here.",
                ));
                lines.push(Line::from(""));
                lines.push(field("Verification code", &session.code));
                lines.push(Line::from(""));
                lines.extend(wrap_text("If the browser didn't open, use this URL:"));
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
        let mut lines = wrap_text(
            "You can walk through the rest of the quickstart yourself, or hand it to a coding \
             agent: picking the agent installs the Svix skills and hands over to them.",
        );
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
        let mut lines =
            wrap_text("The skills are installed for every coding agent found on this machine:");
        lines.push(Line::from(""));
        lines.extend(self.install_lines());
        lines.push(Line::from(""));

        // Once the scope is picked there's nothing left to ask: either a run is in flight,
        // or one of them failed and the error below says so. A finished run doesn't get
        // here at all, since the wizard quits to hand over to the agent.
        if self.scope.is_some() {
            return lines;
        }

        lines.extend(wrap_text("Where should they go?"));
        lines.push(Line::from(""));
        lines.extend(choices(SCOPE_CHOICES, self.scope_selected));

        lines
    }

    /// One line per skill, marked with how far the run has got.
    ///
    /// Nothing is running until the scope is picked, so until then every skill is listed
    /// as queued rather than the first one looking like it's already going.
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
        let mut lines = wrap_text(
            "Pick the language you'll be integrating in. The code samples in the rest of the \
             quickstart are shown in it.",
        );
        lines.push(Line::from(""));
        lines.extend(choices(
            &LANGUAGES
                .iter()
                .map(|language| language.name)
                .collect::<Vec<_>>(),
            self.language,
        ));

        lines
    }

    fn application_lines(&self, width: u16) -> Vec<Line<'static>> {
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
        lines.extend(wrap_text(
            "Create environment-specific tokens in the dashboard: https://dashboard.svix.com",
        ));
        lines.push(Line::from(""));
        lines.push(Line::styled("3. Send a message from your code", HEADING));
        lines.push(Line::from(""));
        lines.extend(wrap_text(
            "Put this where the event actually happens. The same spot you'd log it or fire an \
             internal event.",
        ));
        lines.push(Line::from(""));

        match self.sample_text() {
            Some((title, syntax, text)) => {
                lines.extend(sample(title, syntax, &text, width));
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

    fn send_lines(&self, width: u16) -> Vec<Line<'static>> {
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
        if let Some((title, syntax, text)) = self.sample_text() {
            lines.extend(sample(title, syntax, &text, width));
        }
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
            (None, Some(_)) => {
                lines.push(Line::styled("Press enter to send it.", HEADING));
                lines.push(Line::styled(UNLOCK_HINT, DIM));
            }
            _ => {}
        }

        lines
    }

    fn portal_lines(&self, width: u16) -> Vec<Line<'static>> {
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
            None => lines.push(Line::styled("No link yet. Press r to generate one.", DIM)),
        }

        lines.push(Line::from(""));
        lines.extend(wrap_text(
            "Links like that are short-lived, so you mint them on demand from your backend and \
             link to them from your own dashboard:",
        ));
        lines.push(Line::from(""));

        if let Some((title, syntax, text)) = self.sample_text() {
            lines.extend(sample(title, syntax, &text, width));
        }
        if let Some(qs) = &self.quickstart {
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
            "Press q to leave. The ids and links from these steps get printed on the way out.",
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

/// A highlighted code sample, drawn as text so it scrolls with everything else.
///
/// The block is a rule above and below plus the highlighting theme's background, with no
/// side borders: selecting the sample with the mouse then picks up the code alone.
fn sample(title: &str, syntax: Syntax, text: &str, width: u16) -> Vec<Line<'static>> {
    let width = (width as usize).max(8);
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
            .map(|line| fit(line, width).style(background())),
    );

    lines.push(Line::styled("─".repeat(width), rule));

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

/// Copies `text` through the terminal itself with OSC 52.
///
/// Going through the terminal is what makes this work over SSH and in a multiplexer,
/// where a clipboard library would reach for the wrong machine's clipboard.
fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
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

/// A pick-one list, rendered inline so it scrolls with the rest of the step.
fn choices(options: &[&'static str], selected: usize) -> Vec<Line<'static>> {
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

/// Prose is wrapped by the paragraph widget, so this just owns the text.
fn wrap_text(text: &str) -> Vec<Line<'static>> {
    vec![Line::styled(text.to_owned(), TEXT)]
}

#[cfg(test)]
mod tests {
    use ratatui::{backend::TestBackend, Terminal};
    use svix::{
        api::Svix,
        models::{ApplicationOut, EndpointOut},
    };

    use super::*;

    /// An app past the login step, which is where most of the wizard's behaviour lives.
    fn app() -> App {
        let mut app = App::new();
        app.cfg = Config::default();
        app.client = Some(Svix::new("testsk_fake".to_owned(), None));
        app.auth = Auth::Done;
        app.pending = None;
        app
    }

    fn quickstart() -> Quickstart {
        Quickstart {
            app: ApplicationOut {
                created_at: Default::default(),
                id: "app_123".to_owned(),
                metadata: Default::default(),
                name: "My first app".to_owned(),
                throttle_rate: None,
                uid: Some("quickstart-abc".to_owned()),
                updated_at: Default::default(),
            },
            endpoint: EndpointOut {
                created_at: Default::default(),
                description: String::new(),
                disabled: None,
                event_types: None,
                id: "ep_123".to_owned(),
                metadata: Default::default(),
                throttle_rate: None,
                uid: None,
                updated_at: Default::default(),
                url: "https://play.svix.com/in/token/".to_owned(),
                channels: None,
            },
            play_view_url: "https://play.svix.com/view/token/".to_owned(),
            channel: None,
        }
    }

    /// Renders every step at a small size, which is where layout and scroll bugs show up.
    #[test]
    fn every_step_renders() {
        // Nothing has run yet, so this is also the "still loading" state of each step.
        let mut app = app();

        for step in 0..STEPS.len() {
            app.step = step;
            assert!(
                screen(&app).contains("quit"),
                "the footer should always show how to move on"
            );
        }
    }

    /// The whole screen as text, for asserting on what a step actually shows.
    fn screen(app: &App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("terminal");
        terminal.draw(|frame| app.render(frame)).expect("draw");

        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    #[test]
    fn the_agent_path_installs_the_skills_without_leaving_the_wizard() {
        let mut app = app();
        app.step = 1;

        // Picking the agent stays on the step rather than dropping out to a shell prompt.
        app.on_key(KeyEvent::from(KeyCode::Down));
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.mode, Some(QuickstartMode::Agent));
        assert_eq!(app.step, 1, "the agent path doesn't walk the manual steps");
        assert!(!app.quit, "and the wizard keeps the screen to ask where");

        // What it asks for is the scope, which enter turns into the install.
        let (options, _) = app.choices().expect("a scope to pick");
        assert_eq!(options, SCOPE_CHOICES);

        // Every skill is named on screen before any of them is installed.
        let rendered = screen(&app);
        assert!(rendered.contains("Where should they go?"));
        for skill in SKILL_NAMES {
            let line = format!("Installing {skill} using npx");
            assert!(
                rendered.contains(&line),
                "{line:?} should be shown, got:\n{rendered}"
            );
        }
        app.on_key(KeyEvent::from(KeyCode::Down));
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert!(matches!(app.scope, Some(SkillScope::Global)));
        assert!(matches!(app.pending, Some(Action::InstallSkills)));
        assert_eq!(app.step, 1);

        // A failed install stays put, so enter is another go rather than the manual steps.
        app.pending = None;
        app.error = Some("npx blew up".to_owned());
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.step, 1);
        assert!(matches!(app.pending, Some(Action::InstallSkills)));
        assert!(app.error.is_none(), "and the old error is cleared");
    }

    #[test]
    fn enter_walks_forward_and_nothing_walks_back() {
        let mut app = app();

        // Already logged in, so the first step is done and enter moves on.
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.step, 1);

        // Then pick how to continue.
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.mode, Some(QuickstartMode::Manual));
        assert_eq!(app.step, 2);

        // Then the language, which is just a list to pick from.
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.step, 3);
        app.pending = None;

        // The application step waits until the application exists.
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(
            app.step, 3,
            "can't move on while the application is pending"
        );
        assert!(
            app.status.is_some(),
            "and the wizard says what it's waiting for"
        );

        app.quickstart = Some(quickstart());
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.step, 4);

        // None of the keys that used to go back do anything now.
        for key in [
            KeyCode::Left,
            KeyCode::Char('h'),
            KeyCode::BackTab,
        ] {
            let step = app.step;
            app.on_key(KeyEvent::from(key));
            assert_eq!(app.step, step, "{key:?} shouldn't move between steps");
        }
    }

    #[test]
    fn the_login_step_takes_a_token_typed_in_by_hand() {
        let mut app = app();
        app.auth = Auth::Choosing { selected: 0 };
        app.client = None;

        // Enter on the first option would start a browser login, so pick the second.
        app.on_key(KeyEvent::from(KeyCode::Down));
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert!(matches!(app.auth, Auth::Typing { .. }));

        // Typing goes into the token rather than being read as wizard keys.
        for c in "sk_q".chars() {
            app.on_key(KeyEvent::from(KeyCode::Char(c)));
        }
        app.on_key(KeyEvent::from(KeyCode::Backspace));
        let Auth::Typing { token } = &app.auth else {
            panic!("still typing");
        };
        assert_eq!(token, "sk_");

        // Escape backs out to the choices instead of quitting the wizard.
        app.on_key(KeyEvent::from(KeyCode::Esc));
        assert!(matches!(app.auth, Auth::Choosing { .. }));
        assert!(!app.quit);
    }

    #[test]
    fn an_unauthenticated_wizard_stops_on_the_login_step() {
        let mut app = app();
        app.auth = Auth::Choosing { selected: 0 };
        app.client = None;

        app.step = 0;
        assert!(!app.done(0));
        assert!(app.blocker().is_some(), "the login step blocks the rest");
    }

    #[test]
    fn the_app_portal_stays_shut_until_the_message_is_sent() {
        let mut app = app();

        let send = STEPS
            .iter()
            .position(|s| *s == Step::Send)
            .expect("send step");
        app.step = send;

        // Enter sends rather than moving on, and only sending gets you to the portal.
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.step, send, "the portal is out of reach before sending");

        app.sent = Some("msg_123".to_owned());
        app.on_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(app.step, send + 1, "sending opens it up");
    }

    #[test]
    fn c_copies_the_step_sample_or_its_link() {
        let mut app = app();
        app.mode = Some(QuickstartMode::Manual);
        app.quickstart = Some(quickstart());
        app.message = Some(SampleMessage {
            event_type: "invoice.paid".to_owned(),
            payload: serde_json::json!({ "id": "invoice_WF7WtC" }),
        });

        app.step = STEPS
            .iter()
            .position(|s| *s == Step::Application)
            .expect("application step");
        let (label, text) = app.copy_target().expect("something to copy");
        assert_eq!(label, "sample");
        assert!(
            text.contains("app_123"),
            "the sample is rendered for this app"
        );
        app.on_key(KeyEvent::from(KeyCode::Char('c')));
        assert!(app.status.is_some(), "copying says what it did");

        // Steps with a link copy that instead, matching what `o` opens.
        app.step = STEPS
            .iter()
            .position(|s| *s == Step::Portal)
            .expect("portal step");
        app.portal = Some("https://app.svix.com/login#key=abc".to_owned());
        let (label, text) = app.copy_target().expect("something to copy");
        assert_eq!(label, "link");
        assert_eq!(text, "https://app.svix.com/login#key=abc");
    }

    #[test]
    fn steps_ahead_of_the_current_one_are_hidden() {
        let mut app = app();
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("terminal");

        let rendered = |app: &App, terminal: &mut Terminal<TestBackend>| -> String {
            terminal.draw(|frame| app.render(frame)).expect("draw");
            terminal
                .backend()
                .buffer()
                .content()
                .iter()
                .map(|cell| cell.symbol())
                .collect()
        };

        let first = rendered(&app, &mut terminal);
        assert!(first.contains("Authenticate"), "the current step is listed");
        assert!(
            !first.contains("App portal"),
            "steps that haven't been reached yet stay out of the list"
        );

        app.step = STEPS.len() - 1;
        let last = rendered(&app, &mut terminal);
        assert!(last.contains("App portal"), "reaching a step lists it");
    }

    #[test]
    fn steps_get_checked_off_as_they_are_done() {
        let mut app = app();

        assert!(STEPS
            .iter()
            .enumerate()
            .filter(|(_, step)| **step != Step::Auth)
            .all(|(i, _)| !app.done(i)));

        app.mode = Some(QuickstartMode::Manual);
        app.quickstart = Some(quickstart());
        app.sent = Some("msg_123".to_owned());
        app.portal = Some("https://app.svix.com/login".to_owned());
        app.step = STEPS.len() - 1;

        for (index, step) in STEPS.iter().enumerate() {
            assert_eq!(
                app.done(index),
                *step != Step::Done,
                "{} should be checked off",
                step.title()
            );
        }
    }

    #[test]
    fn the_language_step_is_a_list_you_pick_from() {
        let mut app = app();

        app.step = STEPS
            .iter()
            .position(|s| *s == Step::Language)
            .expect("language step");
        app.on_key(KeyEvent::from(KeyCode::Down));
        assert_eq!(app.language, 1);
        app.on_key(KeyEvent::from(KeyCode::Up));
        assert_eq!(app.language, 0);
        // The list doesn't wrap around at either end.
        app.on_key(KeyEvent::from(KeyCode::Up));
        assert_eq!(app.language, 0);
        assert_eq!(app.scroll, 0, "the vertical keys choose here, not scroll");

        // Off the language step they scroll instead, and nothing changes the language.
        app.step = STEPS
            .iter()
            .position(|s| *s == Step::Send)
            .expect("send step");
        app.on_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(app.scroll, 1);
        assert_eq!(app.language, 0);
    }
}
