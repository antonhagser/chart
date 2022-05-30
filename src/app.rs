use yew::{function_component, html};

use crate::components::chart::Chart;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class="app">
                <h1>{ "😎" }</h1>
                <Chart color={(0.1, 0.2, 0.1)} />
                <Chart color={(1.0, 0.2, 0.1)} />
                <Chart color={(0.3, 0.6, 0.1)} />
                <Chart color={(0.3, 0.6, 1.0)} />
            </div>
        </main>
    }
}
