//! The full-screen quickstart.
//!
//! The steps are a checklist you work down: each one builds on the one before it, so the
//! wizard only moves forward, ticking steps off as they're done. The only key that
//! changes anything on your account is Enter.
//!
//! This file owns the state and what the keys do to it; [`render`] draws it, and
//! [`widgets`] holds the drawing helpers the steps share.

mod render;
mod widgets;

use std::{
    cell::Cell,
    sync::LazyLock,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};
use svix::api::Svix;

use self::widgets::copy_to_clipboard;
use super::{
    create_application, highlight::Syntax, install_skill, portal_url, sample_message, send_message,
    Language, Quickstart, QuickstartMode, SampleMessage, SkillScope, DEFAULT_SERVER_URL, LANGUAGES,
    SKILL_NAMES,
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

impl Default for Auth {
    fn default() -> Self {
        Self::Choosing { selected: 0 }
    }
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

/// The names in `LANGUAGES`, in a shape the choice lists can borrow.
static LANGUAGE_NAMES: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| LANGUAGES.iter().map(|language| language.name).collect());

#[derive(Default)]
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
            width: Cell::new(80),
            auth: if authenticated {
                Auth::Done
            } else {
                Auth::default()
            },
            ..Self::default()
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
    fn choices(&self) -> Option<(&'static [&'static str], usize)> {
        match (self.step(), &self.auth) {
            (Step::Auth, Auth::Choosing { selected }) => Some((AUTH_CHOICES, *selected)),
            (Step::Mode, _) if self.mode.is_none() => Some((MODE_CHOICES, self.mode_selected)),
            // The agent path stays on this step to ask where the skills should go.
            (Step::Mode, _) if self.mode == Some(QuickstartMode::Agent) && self.scope.is_none() => {
                Some((SCOPE_CHOICES, self.scope_selected))
            }
            (Step::Language, _) => Some((LANGUAGE_NAMES.as_slice(), self.language)),
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
            Step::Mode => self.mode_selected = next,
            _ => {}
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

#[cfg(test)]
mod tests {
    use ratatui::{backend::TestBackend, Terminal};
    use svix::{
        api::Svix,
        models::{ApplicationOut, EndpointOut},
    };

    use super::*;

    fn step_index(step: Step) -> usize {
        STEPS.iter().position(|s| *s == step).expect("a known step")
    }

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
        for key in [KeyCode::Left, KeyCode::Char('h'), KeyCode::BackTab] {
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

        let send = step_index(Step::Send);
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

        app.step = step_index(Step::Application);
        let (label, text) = app.copy_target().expect("something to copy");
        assert_eq!(label, "sample");
        assert!(
            text.contains("app_123"),
            "the sample is rendered for this app"
        );
        app.on_key(KeyEvent::from(KeyCode::Char('c')));
        assert!(app.status.is_some(), "copying says what it did");

        // Steps with a link copy that instead, matching what `o` opens.
        app.step = step_index(Step::Portal);
        app.portal = Some("https://app.svix.com/login#key=abc".to_owned());
        let (label, text) = app.copy_target().expect("something to copy");
        assert_eq!(label, "link");
        assert_eq!(text, "https://app.svix.com/login#key=abc");
    }

    #[test]
    fn steps_ahead_of_the_current_one_are_hidden() {
        let mut app = app();

        let first = screen(&app);
        assert!(first.contains("Authenticate"), "the current step is listed");
        assert!(
            !first.contains("App portal"),
            "steps that haven't been reached yet stay out of the list"
        );

        app.step = STEPS.len() - 1;
        let last = screen(&app);
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

        app.step = step_index(Step::Language);
        app.on_key(KeyEvent::from(KeyCode::Down));
        assert_eq!(app.language, 1);
        app.on_key(KeyEvent::from(KeyCode::Up));
        assert_eq!(app.language, 0);
        // The list doesn't wrap around at either end.
        app.on_key(KeyEvent::from(KeyCode::Up));
        assert_eq!(app.language, 0);
        assert_eq!(app.scroll, 0, "the vertical keys choose here, not scroll");

        // Off the language step they scroll instead, and nothing changes the language.
        app.step = step_index(Step::Send);
        app.on_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(app.scroll, 1);
        assert_eq!(app.language, 0);
    }
}
