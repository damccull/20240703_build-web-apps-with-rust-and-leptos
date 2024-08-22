use leptos::*;

#[component]
pub fn ChildWriteSignal(counter: ReadSignal<i32>, set_counter: WriteSignal<i32>) -> impl IntoView {
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);
    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px;">
                <h3>"Child Write Signal"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=increment_counter>"Child +1"</button>
                    <button type="button" on:click=decrement_counter>"Child -1"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ParentWriteSignal() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px;">
                <h3>"Parent Write Signal"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=increment_counter>"Parent +1"</button>
                    <button type="button" on:click=decrement_counter>"Parent -1"</button>
                </div>
                <div>
                    <ChildWriteSignal counter set_counter/>
                </div>
            </div>
        </div>
    }
}
