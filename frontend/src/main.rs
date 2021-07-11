use backend::prelude as pyc;
use yew::prelude::*;

fn main() {
    // this traces on panic in the js console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // mount Polycentrics to body of index.html
    yew::start_app::<Polycentrics>();
}

/// Holds game and UI state.
struct Polycentrics {
    // `ComponentLink` is like a reference to a component
    // it can be used to send messages to the component
    link: ComponentLink<Self>,
    board: pyc::Board,
}

/// Attributes a [`Polycentrics`] game can get from Html.
#[derive(Clone, Properties)]
struct GameProps {
    // number of points on the board in width and height
    size: u8,
    // does not need "render only board, not tile pad"
    // because its deduced from `board.state`
    // maybe later first player, and preconfiguration hash
}

impl Default for GameProps {
    fn default() -> Self {
        GameProps { size: 11 }
    }
}

/// UI messages.
enum GameMsg {
    // render SVG board
    RenderBoard,
    // set tile given index
    SetTile(usize),
}

impl Component for Polycentrics {
    type Message = GameMsg;
    type Properties = GameProps;

    /// Render new element of [`Polycentrics`] to [`Html`].
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // inital drawcall
        link.send_message(Self::Message::RenderBoard);

        Self {
            link,
            board: pyc::Board {
                // TODO: implement once backend is ready
            },
        }
    }

    /// Updates the UI based on an action message [`GameMsg`].
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::RenderBoard => {
                // re-renders automatically with view when `ShouldRender == true`
                // update dom
                true
            }
            Self::Message::SetTile(tile) => {
                // place tile on board
                self.board.step(tile);

                // rerender the board by updating view
                self.link.send_message(Self::Message::RenderBoard);

                // because view is now already refreshed, do not update again
                false
            }
        }
    }

    /// Default set to false, as game properties can not change while playing.
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    // Rendered will do nothing by default, so no implementation

    /// Render [`Html`] view of [`Polycentrics`].
    fn view(&self) -> Html {
        html! {
            <div class="polycentrics">
                // { self.board_view() }
                // { self.tile_pad_view() }
            </div>
        }
    }
}
