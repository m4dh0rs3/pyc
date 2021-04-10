use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use backend::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Polycentrics>::new().mount_to_body();
}

/// Holds game and UI state.
struct Polycentrics {
    link: ComponentLink<Self>,
    board: Board,
    canvas_ref: NodeRef,
    canvas_size: u16,
    canvas_scale: f64,
    canvas_offset: f64,
}

/// UI messages.
enum Msg {
    DrawBoardCanas,
    SetTile(usize),
}

/// Properties a [`Polycentrics`] game can
/// have from html.
#[derive(Clone, Properties)]
struct GameProps {
    canvas_size: u16,
}

impl Default for GameProps {
    fn default() -> Self {
        Self { canvas_size: 400 }
    }
}

/// Implemen [`Polycentrics`] as an html element.
impl Component for Polycentrics {
    type Message = Msg;
    type Properties = GameProps;

    /// Returns new element as html.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // first draw call
        link.send_message(Msg::DrawBoardCanas);

        Self {
            link,
            board: Board::default(),
            canvas_ref: NodeRef::default(),
            canvas_size: props.canvas_size,
            canvas_scale: props.canvas_size as f64 / 11.0,
            canvas_offset: props.canvas_size as f64 / 22.0,
        }
    }

    /// Updates the UI based on an action message.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DrawBoardCanas => {
                self.draw();

                true
            }
            Msg::SetTile(tile) => {
                self.board.step(tile);

                self.link.send_message(Msg::DrawBoardCanas);
                false
            }
        }
    }

    /// Default set to false, as ingame properties should not change.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    /// Return html.
    fn view(&self) -> Html {
        html! {
            <div class="polycentrics">
                { self.tiles_nodes_view() }
                { self.board_canvas_view() }
                { self.tile_pad_view() }
            </div>
        }
    }
}
