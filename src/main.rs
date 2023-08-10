use leptos::html::Div;
use leptos::*;
use leptos_use::use_element_visibility;

#[component]
fn Visibility() -> impl IntoView {
    let el = create_node_ref::<Div>();

    let is_visible = use_element_visibility(el);

    view! {
        <div>
            <div node_ref=el class="max-w-lg relative area dark:bg-gray-800 shadow-lg z-60">
                "Target Element which should be marked as visible: "
                {is_visible}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <p>"Happy Leptos world!"</p>
                <Visibility/>
            </Suspense>
        }
    })
}
