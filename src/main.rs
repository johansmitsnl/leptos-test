use leptos::{
    logging::{debug_warn, warn},
    prelude::*,
};
use reactive_stores::{Field, Store};
use serde::Serialize;

#[derive(Clone, Serialize, Store)]
pub(crate) struct Item {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Clone, Serialize, Store)]
pub(crate) struct Config {
    #[store(key: String = |row| row.key.clone())]
    pub(crate) schema: Vec<Item>,
}

#[component]
fn App(config_store: Store<Config>) -> impl IntoView {
    move || {
        view! {
            <ul>
                <For
                    each=move || config_store.schema()
                    key=|row| row.read().key.clone()
                    children=|item| {
                        let key_label = item.clone().key();
                        let value = item.clone().value();

                        view! {
                            <li>
                                <label>{move || key_label.get()}</label>
                                <br />
                                <Input value />
                            </li>
                        }
                    }
                />
            </ul>
            <hr />
            <h2>"Data representation"</h2>
            <pre>
                {move || {
                    serde_json::to_string_pretty(&config_store.get())
                        .unwrap_or_else(|_| "Error generating a json".to_string())
                }}
            </pre>
        }
    }
}

#[component]
fn Input(#[prop(into)] value: Field<String>) -> impl IntoView {
    let is_empty = Signal::derive(move || value.get().is_empty());

    view! {
        <input type="text" bind:value=value />
        <br />
        "Empty: "
        {move || is_empty.get()}
    }
}

fn main() {
    let schema = vec![Item {
        key: "my-key".to_string(),
        value: "initial".to_string(),
    }];
    let initial_config = Config { schema };
    let config_store = Store::new(initial_config);

    mount_to_body(move || {
        view! {
            <h1>"APP"</h1>
            <button on:click=move |_| {
                debug_warn!("Update value of the first item with a new value");
                config_store
                    .schema()
                    .update(|items| {
                        *items = vec![
                            Item {
                                key: "my-key".to_string(),
                                value: "new_value".to_string(),
                            },
                        ];
                    });
            }>Change it</button>
            <button on:click=move |_| {
                debug_warn!("Remove value of the key with a new value");
                config_store
                    .schema()
                    .update(|items| {
                        items.remove(0);
                    });
            }>Remove it</button>
            <button on:click=move |_| {
                debug_warn!("Clear");
                config_store.set(Config { schema: Vec::new() });
            }>Clear</button>
            <hr />
            <App config_store />
        }
    })
}
