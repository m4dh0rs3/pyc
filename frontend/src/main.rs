use yew::prelude::*;

fn main() {
    // this traces on panic in the js console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::start_app::<Polycentrics>();
}

enum Msg {}

#[derive(Properties, Clone)]
struct Props {}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}

struct Polycentrics {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for Polycentrics {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>{ "Test" }</div>
        }
    }
}
