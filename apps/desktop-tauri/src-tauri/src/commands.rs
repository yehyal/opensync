use storage::db::Event;
use tauri::State;

use crate::state::{AppState, AuthState};

#[tauri::command]
pub fn is_authenticated(state: State<'_, AuthState>) -> bool {
    state.is_logged_in()
}

#[tauri::command]
pub fn get_events(state: State<'_, AppState>) -> Vec<Event> {
    let db = state.db().lock().unwrap();

    let events = db.get_events().unwrap();

    events
}
