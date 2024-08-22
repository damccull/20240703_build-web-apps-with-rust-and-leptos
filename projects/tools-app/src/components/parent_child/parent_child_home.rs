use leptos::*;
use leptos_router::*;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    view! {
        <div>
            <h2>"Parent Child Home"</h2>
            <Outlet/>
        </div>
    }
}
