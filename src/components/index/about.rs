use crate::router::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="about-container">
                <div class="about-left-side">
                    <img class="about-image" src="images/rust-2018.jpeg" />
                </div>
                <div class="about-right-side">
                    <p class="about-description">
                        {"Desde 2016 hemos organizado eventos en la ciudad."}
                        <br/>
                        {"Dando clases en universidades, charlas en eventos y acercando oradores del exterior para dar a conocer el lenguaje."}
                    </p>
                    <RouterAnchor<AppRoute> classes="about-action" route=AppRoute::About>
                                {"Acerca"}
                    </RouterAnchor<AppRoute>>
                </div>
            </section>
        }
    }
}
