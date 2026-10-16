use std::sync::Mutex;

use tauri::State;
use worldgen::{GenerationView, SimConfig, World, advance, generate, generation_view};

struct AppState {
    world: Mutex<Option<World>>,
}

#[tauri::command]
fn generate_world_chronicle(state: State<'_, AppState>) -> GenerationView {
    let world = generate(SimConfig::default());
    let view = generation_view(&world);
    *state.world.lock().expect("app state") = Some(world);
    view
}

#[tauri::command]
fn continue_world_chronicle(
    state: State<'_, AppState>,
    years: u32,
) -> Result<GenerationView, String> {
    let mut guard = state
        .world
        .lock()
        .map_err(|_| "Stan aplikacji zajęty.".to_string())?;
    let world = guard
        .as_mut()
        .ok_or_else(|| "Brak wygenerowanego świata — najpierw wygeneruj kronikę.".to_string())?;
    advance(world, years);
    Ok(generation_view(world))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            world: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            generate_world_chronicle,
            continue_world_chronicle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
