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

/// Attributes a [`Polycentrics`] game can get from Html.
#[derive(Clone, Properties)]
struct GameProps {
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
    RenderBoard,
    SetTile(u8),
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
    // not a component to reduce complexity.
    // if you want non-interactive, set `Board.state`
    // to `State::Draw | State::Victory(_)`
    fn board_view(&self) -> Html {
        html! {
            <svg
                // could be optimized to only render to string once,
                // but needs to be cloned anyway
                width=self.board.points.len().to_string()
                height=self.board.points.len().to_string()
                viewBox=format!("-1 -1 {0} {0}", self.board.points.len() + 1)
                xmlns="http://www.w3.org/2000/svg">
                { self.board_points() }
            </svg>
        }
    }

    /// Render the points of [`Board`] to SVG.
    // this cant be static as points can change color
    fn board_points(&self) -> Html {
        // iter through all the points
        self.board
            .points
            .iter()
            .enumerate()
            .map(|(i, points)| {
                points.iter().enumerate().map(move |(j, point)| {
                    html! {
                        // set position to the indecies
                        <circle cx=i.to_string() cy=j.to_string() r="0.1" fill=match point {
                            // match the point color
                            Some(player) => match player {
                                Player::Gamma => "red",
                                Player::Delta => "blue",
                            },
                            None => "black"
                        } />
                    }
                })
            })
            .flatten()
            .collect()
    }

    /// [`Html`] view of the tile pad.
    // not a component because not sure how yew handles components
    // communication for tile click. also destroy on update
    fn tile_pad_view(&self) -> Html {
        html! {}
    }
}
