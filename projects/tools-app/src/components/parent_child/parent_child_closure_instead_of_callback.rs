use ev::MouseEvent;
use leptos::*;

#[component]
pub fn ParentChildClosure() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 2);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px;">
                <h3>"Parent Closure Instead of Callback"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=increment_counter>"Parent +"</button>
                    <button type="button" on:click=decrement_counter>"Parent -"</button>
                </div>
                <div>
                        <Child counter on_increment=increment_counter on_decrement=decrement_counter/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Child<IncrementButton, DecrementButton>(
    counter: ReadSignal<i32>,
    on_increment: IncrementButton,
    on_decrement: DecrementButton,
) -> impl IntoView
where
    IncrementButton: Fn(MouseEvent) + 'static,
    DecrementButton: Fn(MouseEvent) + 'static,
{
    view! {
        <div style="border: 1px solid black; margin: 4px;">
            <h3>"Child Callback"</h3>
            <p>"Counter: " {counter}</p>
            <div>
                <button type="button" on:click=on_increment>"Parent +"</button>
                <button type="button" on:click=on_decrement>"Parent -"</button>
            </div>
        </div>
    }
}
