use leptos::*;
use leptos_router::Outlet;
use leptos_router::ProtectedRoute;
use leptos_router::Redirect;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;
use leptos_router::RoutingProgress;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Token {
    pub id: i32,
    pub account_id: i32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AppState {
    pub token: Option<Token>,
}

async fn fetch_token() -> Result<Token, ()> {
    Ok(Token {
        id: 1,
        account_id: 1,
    })
}

#[component]
fn App() -> impl IntoView {
    let (is_routing, set_is_routing) = create_signal(false);
    let app_state = expect_context::<RwSignal<AppState>>();

    let (token, _) = create_slice(
        // we take a slice *from* `state`
        app_state,
        // our getter returns a "slice" of the data
        |state| state.token,
        // our setter describes how to mutate that slice, given a new value
        |state, token| state.token = token,
    );

    view! {
      <RoutingProgress
        is_routing
        max_time=std::time::Duration::from_millis(250)
        class="RoutingProgress"
      />
      <Router set_is_routing>
        <Routes>
          <ProtectedRoute
            path="/login"
            redirect_path="/dash"
            condition=move || token().is_none()
            view=|| view! { "Login page" }
          />
          <ProtectedRoute
            path="/*"
            redirect_path="/login"
            condition=move || token().is_some()
            view=|| view! { <p>"Dash page"</p><Outlet/> }
          >
            <Route path="" view=|| view! {  <Redirect path="/dash"/> }/>
            <Route path="/dash" view=|| view! {  <p>"I'm on the dash page now"</p> }/>
          </ProtectedRoute>
        </Routes>
      </Router>
    }
}

#[component]
fn Main() -> impl IntoView {
    let app_state: RwSignal<AppState> = create_rw_signal(AppState { token: None });
    provide_context(app_state);

    let token = create_resource(
        || (),
        move |_| async move {
            log::debug!("loading token from API");
            match fetch_token().await {
                Ok(token) => {
                    app_state.set(AppState { token: Some(token) });
                    Some(token)
                }
                Err(_) => None,
            }
        },
    );

    view! {
      <Suspense fallback=move || {
          view! {  "Loading..." }
      }>{move || { token.read().map(|_| view! {  <App/> }) }}</Suspense>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    mount_to_body(|| view! {  <Main/> })
}
