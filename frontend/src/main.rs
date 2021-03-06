use backend::prelude as pyc;
use yew::prelude::*;

// use `wee_alloc` as the global allocator
// TODO: decide if the 4kB are worth the slowness
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    // this traces on panic in the js console
    #[cfg(debug_assertions)]
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
    // TODO: later first player, and pre configuration hash
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
        // initial draw call
        link.send_message(Self::Message::RenderBoard);

        Self {
            link,
            board: pyc::Board {
                arrow: pyc::Arrow {
                    pos: pyc::Vec2D {
                        x: props.size as i8 / 2,
                        y: props.size as i8 / 2,
                    },
                    dir: pyc::Direction::North,
                },
                points: vec![vec![None; props.size as usize]; props.size as usize],
                ..Default::default()
            },
        }
    }

    /// Updates the UI based on an action message [`GameMsg`].
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::RenderBoard => {
                // re renders automatically with view when `ShouldRender == true`
                // update dom
                true
            }
            Self::Message::SetTile(tile) => {
                // place tile on board
                self.board.step(tile);

                // re render the board by updating view
                self.link.send_message(Self::Message::RenderBoard);

                // because view is now already refreshed, do not update again
                false
            }
        }
    }

    /// Default set to false, as game properties can not change while playing.
    // TODO: reference to `self` is not needed, so `_`?
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    // # rendered will do nothing by default, so no implementation

    /// Render [`Html`] view of [`Polycentrics`].
    fn view(&self) -> Html {
        html! {
            <div class="polycentrics">
                { self.board_view() }
                { self.tile_pad_view() }
            </div>
        }
    }
}

// # HTML views

impl Polycentrics {
    /// SVG view of the board.
    // not a component to reduce complexity.
    // if you want non interactive, set `Board.state`
    // to `State::Draw | State::Victory(_)`
    fn board_view(&self) -> Html {
        html! {
            <svg
                class="board"
                xmlns="http://www.w3.org/2000/svg"

                // could be optimized to only render to string once,
                // but needs to be cloned anyway
                width=self.board.points.len().to_string()
                height=self.board.points.len().to_string()

                // TODO: maybe 0.5 looks better?
                viewBox=format!("-1 -1 {0} {0}", self.board.points.len() + 1)
            >
                // reversed stack draw
                // { self.midpoint_svg() } // DEBUG VIEW!
                // { self.intersections_svg() } // DEBUG VIEW!
                { self.path_svg() }
                { self.points_svg() }
                { self.arrow_svg() }
            </svg>
        }
    }

    /// [`Html`] view of the tile pad.
    // not a component because not sure how yew handles components
    // communication for tile click, also destroy on update
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
                            // TODO: replace with SVG view
                            <button class="tile-button" onclick=self.link.callback(move |_| GameMsg::SetTile(i))>
                                <svg
                                    class="tile"
                                    xmlns="http://www.w3.org/2000/svg"

                                    width=3
                                    height=3
                                    
                                    viewBox=format!("{} {} {} {}", if curve.end.x > 0 { -0.2 } else { -3.2 }, if curve.end.y > 0 { -0.2 } else { -3.2 }, 3.4, 3.4)
                                >
                                    <path
                                        class="curve"
                                        d=format!("M 0 0 Q {} {} {} {}", curve.mid.x, curve.mid.y, curve.end.x, curve.end.y)
                                    />
                                </svg>
                            </button>
                        }
                    })
                    .collect::<Html>()
            }</div>
        }
    }
}

// # SVG

impl Polycentrics {
    /// Render the points of [`Board`] to SVG.
    // this cant be static as points can change color
    fn points_svg(&self) -> Html {
        // iter through all the points
        self.board
            .points
            .iter()
            .enumerate()
            .map(|(j, points)| {
                points.iter().enumerate().map(move |(i, point)| {
                    html! {
                        <circle class=match point {
                                // match the point state
                                // using `classes!` for multiple classes
                                Some(player) => classes!("point", match player {
                                    pyc::Player::Gamma => "point-gamma",
                                    pyc::Player::Delta => "point-delta",
                                }),
                                None => classes!("point")
                            }
                            // set position to the indices
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
                // let circ = curve.radius as f64 * TAU;

                html! {
                    <path
                        class="curve"
                        d=format!("M {} {} Q {} {} {} {}", curve.start.x, curve.start.y, curve.mid.x, curve.mid.y, curve.end.x, curve.end.y)
                    />
                    
                    // not using path arc because its more complex
                    // deprecated since v0.5.0
                    /* <circle
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
                    /> */
                }
            })
            .collect()
    }

    /* /// Render the midpoints of the [`Curve`] to SVG.
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
    } */

    /// Render the arrow of [`Board`] to SVG.
    fn arrow_svg(&self) -> Html {
        // use std::f32::consts::TAU;

        html! {
            // TODO: something fancier than line
            <line
                class="arrow"
                // set position
                x1=self.board.arrow.pos.x.to_string() y1=self.board.arrow.pos.y.to_string()
                // the arrow statically points to the right,
                x2={ self.board.arrow.pos.x as f32 + (match self.board.arrow.dir {
                    pyc::Direction::North | pyc::Direction::South => 0,
                    pyc::Direction::West => -1,
                    pyc::Direction::East => 1,
                } as f32) / 2.0 }.to_string()
                y2={ self.board.arrow.pos.y as f32 + (match self.board.arrow.dir {
                    pyc::Direction::West | pyc::Direction::East => 0,
                    pyc::Direction::North => -1,
                    pyc::Direction::South => 1,
                } as f32) / 2.0 }.to_string()
                /* // and than gets rotated
                transform=format!("rotate({} {} {})", 360.0 * self.board.arrow.angle.0 / TAU, &self.board.arrow.position.x, &self.board.arrow.position.y) */
            />
        }
    }

    /* /// Render intersections points with latest tile.
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
    } */
}