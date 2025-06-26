use crate::data::StrategyData;
use apca::Client;
use std::collections::HashMap;

/// The main application state.
pub struct App {
    pub running: bool,
    pub strategies: HashMap<String, StrategyData>,
    pub clients: HashMap<String, Client>,
}

impl App {
    /// Creates a new App.
    pub fn new(clients: HashMap<String, Client>) -> Self {
        Self {
            running: true,
            strategies: HashMap::new(),
            clients,
        }
    }

    /// Sets the running flag to false, causing the main loop to exit.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // Add methods for handling events, updates, etc.
} 