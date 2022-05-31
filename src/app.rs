use std::collections::VecDeque;

use js_sys::Array;
use rand::Rng;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use yew::{html, Callback, Component, Html};

use crate::components::chart::Chart;

pub enum Msg {
    AddChart((f32, f32, f32)),
    PopFrontChart,
}

pub struct App {
    colors: VecDeque<(Uuid, (f32, f32, f32))>,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        App {
            colors: VecDeque::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let add_onclick = ctx.link().callback(|_| {
            let mut rng = rand::thread_rng();
            let color = (
                rng.gen_range(0.0..0.09),
                rng.gen_range(0.0..0.09),
                rng.gen_range(0.0..0.09),
            );

            Msg::AddChart(color)
        });

        let pop_onclick = ctx.link().callback(|_| Msg::PopFrontChart);

        html! {
            <main>
                <div class="app">
                    <button onclick={add_onclick}>{"Add Color"}</button>
                    <button onclick={pop_onclick}>{"Pop Color"}</button>

                    <Chart color={(0.1, 1.0, 0.6)} />

                    {
                        self.colors.iter().map(|color| {
                            html!{
                                <Chart key={color.0.to_string()} color={color.clone().1} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </main>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        log::warn!("update!");

        match msg {
            Msg::AddChart(c) => {
                self.colors.push_back((Uuid::new_v4(), c));
                true
            }
            Msg::PopFrontChart => {
                self.colors.pop_front();
                true
            }

            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}
