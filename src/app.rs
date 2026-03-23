use std::io::{self, stdout};
use std::panic::{set_hook, take_hook};

use anyhow::{Context, Result};
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::ui;

// -- Types ------------------------------------------------------------------

pub struct AppState {
    pub should_quit: bool,
    pub active_panel: Panel,
}

#[allow(dead_code)]
pub enum Panel {
    Files,
    Diff,
    Comments,
}

#[allow(dead_code)]
pub enum AppEvent {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
}

// -- Public entry point -----------------------------------------------------

pub async fn run() -> Result<()> {
    init_panic_hook();
    let mut terminal = init_terminal()?;

    let result = run_event_loop(&mut terminal).await;

    if let Err(e) = restore_terminal() {
        eprintln!("warning: failed to restore terminal: {e}");
    }

    result
}

// -- Event loop -------------------------------------------------------------

async fn run_event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut state = AppState {
        should_quit: false,
        active_panel: Panel::Files,
    };

    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);

    // Spawn a task that reads crossterm events and forwards them.
    let event_reader = tokio::spawn(read_crossterm_events(tx));

    // Initial draw before any events arrive.
    terminal.draw(|frame| ui::render(frame, &state))?;

    while let Some(event) = rx.recv().await {
        handle_event(&mut state, event);
        terminal.draw(|frame| ui::render(frame, &state))?;

        if state.should_quit {
            break;
        }
    }

    if !state.should_quit {
        // Channel closed unexpectedly — check whether the event reader failed.
        match event_reader.await {
            Ok(Ok(())) => anyhow::bail!("event stream closed unexpectedly"),
            Ok(Err(e)) => return Err(e.context("crossterm event reader failed")),
            Err(e) => anyhow::bail!("event reader task panicked: {e}"),
        }
    }

    Ok(())
}

async fn read_crossterm_events(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    let mut stream = EventStream::new();
    while let Some(result) = stream.next().await {
        let app_event = match result {
            Ok(Event::Key(key)) => AppEvent::Key(key),
            Ok(Event::Resize(w, h)) => AppEvent::Resize(w, h),
            Ok(_) => continue,
            Err(e) => return Err(e).context("crossterm event stream error"),
        };
        if tx.send(app_event).await.is_err() {
            // Receiver dropped — main loop exited.
            break;
        }
    }
    Ok(())
}

// -- Event handling ---------------------------------------------------------

fn handle_event(state: &mut AppState, event: AppEvent) {
    match event {
        AppEvent::Key(key) if key.kind == KeyEventKind::Press => handle_key(state, key),
        // Resize is handled automatically by ratatui on the next draw call.
        _ => {}
    }
}

fn handle_key(state: &mut AppState, key: crossterm::event::KeyEvent) {
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c')) => state.should_quit = true,
        (KeyModifiers::NONE, KeyCode::Char('q')) => state.should_quit = true,
        _ => {}
    }
}

// -- Terminal setup / teardown ----------------------------------------------

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        if let Err(e) = restore_terminal() {
            eprintln!("warning: failed to restore terminal during panic: {e}");
            eprintln!("you may need to run `reset` to fix your terminal");
        }
        original_hook(panic_info);
    }));
}
