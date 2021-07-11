use std::f64::consts::TAU;

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
                class="board"
                // could be optimized to only render to string once,
                // but needs to be cloned anyway
                width=self.board.points.len().to_string()
                height=self.board.points.len().to_string()
                viewBox=format!("-1 -1 {0} {0}", self.board.points.len() + 1)
                xmlns="http://www.w3.org/2000/svg">
                { self.arrow_svg() }
                // { self.midpoint_svg() } // DEBUG VIEW!
                { self.intersections_svg() } // DEBUG VIEW!
                { self.path_svg() }
                { self.points_svg() }
            </svg>
        }
    }

    /// Render the points of [`Board`] to SVG.
    // this cant be static as points can change color
    fn points_svg(&self) -> Html {
        // iter through all the points
        self.board
            .points
            .iter()
            .enumerate()
            .map(|(i, points)| {
                points.iter().enumerate().map(move |(j, point)| {
                    html! {
                        <circle class=match point {
                                // match the point state
                                // using `classes!` for multiple classes
                                Some(player) => classes!("point", match player {
                                    Player::Gamma => "point-gamma",
                                    Player::Delta => "point-delta",
                                }),
                                None => classes!("point")
                            }
                            // set position to the indecies
                            cx=i.to_string() cy=j.to_string()
                            // the [geometric property `r` of svg 2](https://svgwg.org/svg2-draft/geometry.html#R) is not currently (88) supported in firefox
                            // TODO: remove when supported, also at `midpoint_svg()`
                            r="0.1"
                        />
                    }
                })
            })
            .flatten()
            .collect()
    }

    /// Render the path of [`Board`] to SVG.
    fn path_svg(&self) -> Html {
        // iter trough all the path elements
        self.board
            .path
            .iter()
            .map(|curve| {
                let circ = curve.radius as f64 * TAU;

                html! {
                    // not using path arc because its more complex
                    <circle
                        class="curve"
                        // set the midpoint and radius
                        cx=curve.mid.x.to_string() cy=curve.mid.y.to_string() r=curve.radius.to_string()
                        // rotate around itself
                        // TODO: check if start angle should be switched with `Curve.dir`
                        transform=format!("rotate({} {} {})", if curve.off.0 > 0.0 {
                            curve.start.into_deg()
                        } else {
                            (curve.start + curve.off).normal().into_deg()
                        }, &curve.mid.x, &curve.mid.y)
                        // draw only `90deg` of the circle
                        style=format!("stroke-dasharray: {} {};", circ * 0.25, circ * 0.75)
                    />
                }
            })
            .collect()
    }

    /// Render the midpoints of the [`Curve`] to SVG.
    // DEBUG VIEW!
    fn midpoint_svg(&self) -> Html {
        self.board
            .path
            .iter()
            .map(|curve| {
                html! {
                    <circle
                        class="midpoint"
                        // set the midpoint and radius
                        cx=curve.mid.x.to_string() cy=curve.mid.y.to_string() r="0.2"
                    />
                }
            })
            .collect()
    }

    /// Render intersections points with latest tile.
    fn intersections_svg(&self) -> Html {
        self.board
            .intersections()
            .iter()
            .map(|(point, _, _, _)| {
                html! {
                    <circle
                        class=classes!("point", "intersection")
                        // set the midpoint and radius
                        cx=point.x.to_string() cy=point.y.to_string() r="0.1"
                    />
                }
            })
            .collect()
    }

    /// Render the arrow of [`Board`] to SVG.
    fn arrow_svg(&self) -> Html {
        html! {
            // TODO: something fancier than line
            <line
                class="arrow"
                // set position
                x1=self.board.arrow.position.x.to_string() y1=self.board.arrow.position.y.to_string()
                // the arrow statically points to the right,
                x2={ self.board.arrow.position.x as f64 + 0.5 }.to_string() y2=self.board.arrow.position.y.to_string()
                // and than gets rotated
                transform=format!("rotate({} {} {})", self.board.arrow.angle.into_deg(), &self.board.arrow.position.x, &self.board.arrow.position.y)
            />
        }
    }

    /// [`Html`] view of the tile pad.
    // not a component because not sure how yew handles components
    // communication for tile click. also destroy on update
    fn tile_pad_view(&self) -> Html {
        html! {
            <div class="tile-pad">{
                // iterate through all tile options
                self.board
                    .options()
                    .iter()
                    // include the index as argument for `Board.step(i)`
                    .enumerate()
                    .map(|(i, curve)| {
                        html! {
                            <button class="tile" onclick=self.link.callback(move |_| GameMsg::SetTile(i as u8))>{ format!("{:?}", curve) }</button>
                        }
                    })
                    .collect::<Html>()
            }</div>
        }
    }
}
