use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Serialize;
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

fn request_chronicle(set_chronicle: WriteSignal<String>, set_busy: WriteSignal<bool>) {
    set_busy.set(true);
    spawn_local(async move {
        let args = js_sys::Object::new();
        let text = invoke("generate_world_chronicle", args.into())
            .await
            .as_string()
            .unwrap_or_else(|| "Nie udało się wygenerować kroniki.".into());
        set_chronicle.set(text);
        set_busy.set(false);
    });
}

fn continue_chronicle(years: u32, set_chronicle: WriteSignal<String>, set_busy: WriteSignal<bool>) {
    set_busy.set(true);
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&ContinueArgs { years }).unwrap_or(JsValue::NULL);
        let result = invoke("continue_world_chronicle", args).await;
        let text = result
            .as_string()
            .unwrap_or_else(|| "Nie udało się kontynuować symulacji.".into());
        set_chronicle.set(text);
        set_busy.set(false);
    });
}

#[component]
pub fn App() -> impl IntoView {
    let (chronicle, set_chronicle) = signal(String::from("Ładowanie kroniki…"));
    let (busy, set_busy) = signal(false);

    Effect::new(move |_| {
        request_chronicle(set_chronicle, set_busy);
    });

    view! {
        <main class="console-app">
            <header class="console-header">
                <h1>"Kronika wsi"</h1>
                <p class="console-sub">"Iteracja 1 — jedna wieś, kontynuowalna symulacja"</p>
                <div class="console-actions">
                    <button
                        type="button"
                        class="console-btn"
                        prop:disabled=move || busy.get()
                        on:click=move |_| request_chronicle(set_chronicle, set_busy)
                    >
                        {move || if busy.get() { "Generowanie…" } else { "Generuj ponownie" }}
                    </button>
                    <button
                        type="button"
                        class="console-btn"
                        prop:disabled=move || busy.get()
                        on:click=move |_| continue_chronicle(5, set_chronicle, set_busy)
                    >
                        "+5 lat"
                    </button>
                </div>
            </header>
            <section class="console-commands" aria-label="Komendy">
                <h2>"Komendy"</h2>
                <ul>
                    <li>
                        <span class="cmd-label">"Uruchomienie"</span>
                        <code>"cargo tauri dev"</code>
                    </li>
                    <li>
                        <span class="cmd-label">"Albo skrypt"</span>
                        <code>"powershell ./scripts/dev.ps1"</code>
                    </li>
                    <li>
                        <span class="cmd-label">"Nowa historia"</span>
                        <code>"przycisk „Generuj ponownie”"</code>
                    </li>
                    <li>
                        <span class="cmd-label">"Kontynuacja"</span>
                        <code>"przycisk „+5 lat”"</code>
                    </li>
                    <li>
                        <span class="cmd-label">"Uwaga"</span>
                        <code>"nie otwieraj 127.0.0.1 w przeglądarce"</code>
                    </li>
                </ul>
            </section>
            <pre class="console-output">{move || chronicle.get()}</pre>
        </main>
    }
}
