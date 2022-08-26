use crate::components::events::event::Event;

use yew::prelude::*;

pub struct Events {}

impl Component for Events {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Events {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section class="events-section">
            <div class="events-title-container">
                <span class="events-title">{"PROXIMOS EVENTOS"}</span>
            </div>
            <div class="events-container-list">
                <hr class="w-full border-b-2 border-gray-200"/>
                <Event title="Necesitamos hacer fetch a la API de meetup" description="Necesitamos hacer fetch a la API de meetup" date="25 de agosto de 2022 16:30 / 21:30"  />
                <hr class="w-full border-b-2 border-gray-200"/>
                <Event title="Necesitamos hacer fetch a la API de meetup" description="Necesitamos hacer fetch a la API de meetup" date="25 de agosto de 2022 16:30 / 21:30" />
                <hr class="w-full border-b-2 border-gray-200"/>
                </div>
          </section>
        }
    }
}
