use yew::prelude::*;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section class="our-projects-section">
                <div class="our-projects-title-container">
                    <span class="our-projects-title">{"NUESTRA COMUNIDAD"}</span>
                </div>
                <div class="about-container">
                    <div class="about-left-side">
                        <img class="about-image" src="images/los-panas.jpeg" />
                    </div>
                    <div class="about-right-side">
                        <p class="about-description">
                            {"La comunidad de RustMX es pequeña pero creciente."}
                            <br />
                            <br />
                            {"Desde 2016 hemos organizado meetups de Rust, dando clases en universidades, charlas en eventos y acercando oradores del exterior para dar a conocer el lenguaje."}
                            <br />
                            <br />
                            {"Si estas interesado o interesada en aprender Rust, o estas llevando a cabo un proyecto con tecnología asociada, te invitamos a acercarte y participar. "}
                            <br />
                            <br />
                            {"El sitio en Meetup punto com de la comunidad es: RustMX"}
                            <br />
                            {"También tenemos un un canal en Telegram."}
                        </p>
                    </div>
                </div>
            </section>
        }
    }
}
