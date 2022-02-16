extern crate telemetrydeck_wasm;
use telemetrydeck_wasm::TelemetryDeck;
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
    telemetry: TelemetryDeck,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            telemetry: TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                // broadcast a signal whenever we handle this message
                self.telemetry.send("addOne", None, None, None);
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
