use yew::prelude::*;

pub struct Hero {}

impl Component for Hero {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Hero {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <main class="hero-container">
                <img src="images/ferris.png" class="w-full object-cover object-center max-w-[500px] mx-auto" />
                <div class="hero-text-container">
                    <h1 class="hero-title">
                    {"¡La comunidad de Rust en Mexico esta creciendo!"}
                    </h1>
                    <h2 class="hero-subtitle">
                    {"Si estas interesado o interesada en aprender Rust, o estas llevando a cabo un proyecto con tecnología asociada, te invitamos a acercarte y participar."}
                    </h2>
                    <h2 class="hero-subtitle">
                    {"Te dejamos:"}
                    </h2>
                </div>
            </main>
        }
    }
}
