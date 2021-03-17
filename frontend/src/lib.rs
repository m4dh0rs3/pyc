use std::f64::consts::TAU;

use math::Vec2D;
use seed::{prelude::*, *};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use backend::*;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Redraw);
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    canvas: ElRef<HtmlCanvasElement>,
    tiles: [bool; 12],
    board: Board,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            canvas: ElRef::default(),
            tiles: [false; 12],
            board: Board::new(Vec2D::new(4.0, 4.0)),
        }
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Redraw,
    Tile(usize),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Redraw => {
            let canvas = model.canvas.get().expect("Could not get canvas");
            let ctx = seed::canvas_context_2d(&canvas);

            ctx.image_smoothing_enabled();

            let width = canvas.width() as f64;
            let height = canvas.height() as f64;

            ctx.clear_rect(0.0, 0.0, width, height);

            let scale = Vec2D::new(width / 11.0, height / 11.0);
            let offset = scale * 0.5;

            ctx.set_stroke_style(&JsValue::from_str("black"));

            for i in 0..=10 {
                for j in 0..=10 {
                    ctx.begin_path();
                    // 11 dots
                    ctx.arc(
                        scale.x * i as f64 + offset.x,
                        scale.y * j as f64 + offset.y,
                        2.,
                        0.,
                        TAU,
                    );
                    ctx.fill();
                }
            }

            for (_, curve) in model.board.graph.edges() {
                /* ctx.begin_path();

                ctx.arc(
                    scale.x * curve.mid.x + offset.x,
                    scale.y * curve.mid.y + offset.y,
                    4.,
                    0.,
                    TAU,
                );

                ctx.fill();

                let start = curve.first();

                ctx.begin_path();

                ctx.arc(
                    scale.x * start.x + offset.x,
                    scale.y * start.y + offset.y,
                    4.,
                    0.,
                    TAU,
                );

                ctx.fill();

                let end = curve.last();

                ctx.begin_path();

                ctx.arc(
                    scale.x * end.x + offset.x,
                    scale.y * end.y + offset.y,
                    10.,
                    0.,
                    TAU,
                );

                ctx.fill(); */

                ctx.begin_path();

                let o: Vec2D = curve.first() * scale + offset;
                ctx.move_to(o.x, o.y);

                let last: Vec2D = curve.last() * scale + offset;

                ctx.set_stroke_style(&JsValue::from_str("red"));
                ctx.line_to(last.x, last.y);

                ctx.stroke();
                ctx.begin_path();

                ctx.set_stroke_style(&JsValue::from_str("black"));

                for p in &curve.path {
                    let p = p.clone() * scale + offset;

                    ctx.line_to(p.x, p.y);
                    ctx.move_to(p.x, p.y);
                }

                ctx.stroke();
            }

            ctx.set_stroke_style(&JsValue::from_str("red"));

            for node in model.board.graph.nodes() {
                ctx.begin_path();

                ctx.arc(
                    scale.x * node.x + offset.x,
                    scale.y * node.y + offset.y,
                    4.,
                    0.,
                    TAU,
                );

                ctx.stroke();
            }

            /* ctx.begin_path();

            ctx.move_to(
                offset.x + model.board.arrow.0.x * scale.x,
                offset.y + model.board.arrow.0.y * scale.y,
            );
            ctx.line_to(
                offset.x + model.board.arrow.0.x * scale.x + 20. * model.board.arrow.1.cos(),
                offset.y + model.board.arrow.0.y * scale.y + 20. * model.board.arrow.1.sin(),
            );

            ctx.stroke(); */

            /* ctx.set_line_width(10.);
            ctx.stroke_rect(75., 140., 150., 110.);
            ctx.fill_rect(130., 190., 40., 60.);
            ctx.begin_path();
            ctx.move_to(50., 140.);
            ctx.line_to(150., 60.);
            ctx.line_to(250., 140.);
            ctx.close_path();
            ctx.stroke(); */
        }

        Msg::Tile(num) => {
            model.tiles[num] = true;

            match num {
                0 => model.board.tile(Tile {
                    dir: TileDir::UpLeft,
                    radius: 1,
                }),
                1 => model.board.tile(Tile {
                    dir: TileDir::UpLeft,
                    radius: 2,
                }),
                2 => model.board.tile(Tile {
                    dir: TileDir::UpLeft,
                    radius: 3,
                }),
                3 => model.board.tile(Tile {
                    dir: TileDir::UpRight,
                    radius: 3,
                }),
                4 => model.board.tile(Tile {
                    dir: TileDir::UpRight,
                    radius: 2,
                }),
                5 => model.board.tile(Tile {
                    dir: TileDir::UpRight,
                    radius: 1,
                }),
                6 => model.board.tile(Tile {
                    dir: TileDir::DownLeft,
                    radius: 1,
                }),
                7 => model.board.tile(Tile {
                    dir: TileDir::DownLeft,
                    radius: 2,
                }),
                8 => model.board.tile(Tile {
                    dir: TileDir::DownLeft,
                    radius: 3,
                }),
                9 => model.board.tile(Tile {
                    dir: TileDir::DownRight,
                    radius: 3,
                }),
                10 => model.board.tile(Tile {
                    dir: TileDir::DownRight,
                    radius: 2,
                }),
                11 => model.board.tile(Tile {
                    dir: TileDir::DownRight,
                    radius: 1,
                }),
                _ => {}
            }

            orders.send_msg(Msg::Redraw);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        canvas!(
            el_ref(&model.canvas),
            attrs!(
                At::Width => px(500),
                At::Height => px(500),
            ),
            style!(
                St::Border => "1px solid black",
            )
        ),
        div!(
            button!(
                "1lu",
                IF!(model.tiles[0] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(0))
            ),
            button!(
                "2lu",
                IF!(model.tiles[1] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(1))
            ),
            button!(
                "3lu",
                IF!(model.tiles[2] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(2))
            ),
            button!(
                "3ru",
                IF!(model.tiles[3] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(3))
            ),
            button!(
                "2ru",
                IF!(model.tiles[4] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(4))
            ),
            button!(
                "1ru",
                IF!(model.tiles[5] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(5))
            ),
        ),
        div!(
            button!(
                "1ld",
                IF!(model.tiles[6] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(6))
            ),
            button!(
                "2ld",
                IF!(model.tiles[7] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(7))
            ),
            button!(
                "3ld",
                IF!(model.tiles[8] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(8))
            ),
            button!(
                "3rd",
                IF!(model.tiles[9] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(9))
            ),
            button!(
                "2rd",
                IF!(model.tiles[10] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(10))
            ),
            button!(
                "1rd",
                IF!(model.tiles[11] => attrs!(At::Disabled => true)),
                ev(Ev::Click, |_| Msg::Tile(11))
            ),
        ),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
