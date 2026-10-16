use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct ContinueArgs {
    years: u32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
struct SettlementView {
    name: String,
    population: u32,
    size: String,
    livelihood: String,
    districts: Vec<String>,
    buildings: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
struct GenerationView {
    chronicle: String,
    settlements: Vec<SettlementView>,
}

fn parse_view(value: JsValue) -> GenerationView {
    serde_wasm_bindgen::from_value(value).unwrap_or_else(|_| GenerationView {
        chronicle: "Nie udało się odczytać wyniku generacji.".into(),
        settlements: Vec::new(),
    })
}

fn request_generation(set_view: WriteSignal<GenerationView>, set_busy: WriteSignal<bool>) {
    set_busy.set(true);
    spawn_local(async move {
        let args = js_sys::Object::new();
        let value = invoke("generate_world_chronicle", args.into()).await;
        set_view.set(parse_view(value));
        set_busy.set(false);
    });
}

fn continue_generation(
    years: u32,
    set_view: WriteSignal<GenerationView>,
    set_busy: WriteSignal<bool>,
) {
    set_busy.set(true);
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&ContinueArgs { years }).unwrap_or(JsValue::NULL);
        let value = invoke("continue_world_chronicle", args).await;
        set_view.set(parse_view(value));
        set_busy.set(false);
    });
}

#[component]
fn SettlementsPanel(settlements: Vec<SettlementView>) -> impl IntoView {
    view! {
        <section class="settlements-panel" aria-label="Osady">
            <h2>"Osady"</h2>
            <p class="settlements-hint">"Uruchomienie: cargo tauri dev"</p>
            <div class="settlements-grid">
                {settlements
                    .into_iter()
                    .map(|s| {
                        view! {
                            <article class="settlement-card">
                                <h3>{s.name.clone()}</h3>
                                <ul class="settlement-facts">
                                    <li>
                                        <span class="fact-label">"Populacja"</span>
                                        <span>{s.population}</span>
                                    </li>
                                    <li>
                                        <span class="fact-label">"Wielkość"</span>
                                        <span>{s.size.clone()}</span>
                                    </li>
                                    <li>
                                        <span class="fact-label">"Zajęcie"</span>
                                        <span>{s.livelihood.clone()}</span>
                                    </li>
                                    <li>
                                        <span class="fact-label">"Dzielnice"</span>
                                        <span>{s.districts.join(", ")}</span>
                                    </li>
                                    <li>
                                        <span class="fact-label">"Budynki"</span>
                                        <span>{s.buildings.join(", ")}</span>
                                    </li>
                                </ul>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (view, set_view) = signal(GenerationView {
        chronicle: String::from("Ładowanie kroniki…"),
        settlements: Vec::new(),
    });
    let (busy, set_busy) = signal(false);

    Effect::new(move |_| {
        request_generation(set_view, set_busy);
    });

    view! {
        <main class="console-app">
            <header class="console-header">
                <h1>"Kronika osad"</h1>
                <p class="console-sub">"4 wsie — życie NPC i więzi w kronice"</p>
                <div class="console-actions">
                    <button
                        type="button"
                        class="console-btn"
                        prop:disabled=move || busy.get()
                        on:click=move |_| request_generation(set_view, set_busy)
                    >
                        {move || if busy.get() { "Generowanie…" } else { "Generuj ponownie" }}
                    </button>
                    <button
                        type="button"
                        class="console-btn"
                        prop:disabled=move || busy.get()
                        on:click=move |_| continue_generation(5, set_view, set_busy)
                    >
                        "+5 lat"
                    </button>
                </div>
            </header>
            {move || {
                let settlements = view.get().settlements;
                view! { <SettlementsPanel settlements=settlements /> }
            }}
            <pre class="console-output">{move || view.get().chronicle}</pre>
        </main>
    }
}
