use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (counter, set_counter) = create_signal(cx, 0);

    view! { cx,
        <div>
            <header>
                <h1>"WishApp Counter"</h1>
            </header>
            <main>
                <p>{move || format!("Counter: {}", counter.get())}</p>
                <button on:click=move |_| set_counter.update(|count| *count += 1)>
                    "Increment"
                </button>
            </main>
        </div>
    }
}
