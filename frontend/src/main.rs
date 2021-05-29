use backend::prelude::*;
use yew::prelude::*;

fn main() {
    // this traces on panic in the js console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // mount Polycentrics to index.html's body
    yew::start_app::<Polycentrics>();
}

/// Holds game and UI state.
struct Polycentrics {
    // `ComponentLink` is like a reference to a component
    // it can be used to send messages to the component
    link: ComponentLink<Self>,
    board: Board,
}

/// UI messages.
enum GameMsg {
    RenderBoard,
    SetTile(u8),
}

/// Attributes a [`Polycentrics`] game can get from Html.
#[derive(Clone, Properties)]
struct GameProps {
    size: u8,
    // maybe later first player, and preconfiguration hash
}

impl Default for GameProps {
    fn default() -> Self {
        GameProps { size: 11 }
    }
}

impl Component for Polycentrics {
    type Message = GameMsg;
    type Properties = GameProps;

    /// Returns new element as [`Html`].
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // first drawcall
        link.send_message(GameMsg::RenderBoard);

        Self {
            link,
            board: Board {
                points: vec![vec![None; props.size as usize]; props.size as usize],
                ..Default::default()
            },
        }
    }

    /// Updates the UI based on an action message [`GameMsg`].
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GameMsg::RenderBoard => {
                // render svg board

                // should update dom
                true
            }
            GameMsg::SetTile(tile) => {
                // step board
                self.board.step(tile);

                // rerender the board
                self.link.send_message(GameMsg::RenderBoard);

                // should not update dom, because it needs to render the board
                // anyway, which updates the dom
                false
            }
        }
    }

    /// Default set to false, as ingame properties should not change.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    /// Return [`Html`] view of [`Polycentrics`].
    fn view(&self) -> Html {
        html! {
            <div class="polycentrics">
                { self.board_view() }
                { self.tile_pad_view() }
            </div>
        }
    }
}

impl Polycentrics {
    /// SVG view of the board.
    fn board_view(&self) -> Html {
        todo!()
    }

    /// [`Html`] view of the tile pad.
    fn tile_pad_view(&self) -> Html {
        todo!()
    }
}
