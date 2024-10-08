use leptos::*;
use leptos_router::ActionForm;

use crate::{
    components::error_fallback::{self, error_fallback},
    models::color::Color,
    repositories::color_repository::RemoveColor,
};

#[component]
pub fn ColorList(
    color_resource: Resource<(usize, usize), Result<Vec<Color>, ServerFnError>>,
    remove_color: Action<RemoveColor, Result<(), ServerFnError>>,
) -> impl IntoView {
    let color_list_view = move || -> Option<Result<View, _>> {
        color_resource.and_then(|colors| {
            colors
                .iter()
                .map(|color| {
                    view! { <ColorListItem color=color.clone() remove_color/> }
                })
                .collect_view()
        })
    };
    view! {
        <Transition fallback=move||view!{<p>"Loading..."</p>}>
            <ErrorBoundary fallback=error_fallback()>
                <ul>
                    {color_list_view}
                </ul>
            </ErrorBoundary>
        </Transition>

    }
}

#[component]
fn ColorListItem(
    color: Color,
    remove_color: Action<RemoveColor, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <li>
            <ActionForm action=remove_color>
                <input type="hidden" name="id" prop:value=color.id />
                {color.name}
                -
                {color.hex_code}
                <button type="submit" style="margin-left:5px;">X</button>
            </ActionForm>
        </li>
    }
}
