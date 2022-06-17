use yew::{html, Component};

use crate::components::chart::Chart;

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        App {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <main>
                <div class="app">
                    <Chart color={(0.1, 0.2, 0.1)} />
                </div>
            </main>
        }
    }
}
