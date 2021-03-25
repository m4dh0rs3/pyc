/* use std::f64::consts::TAU;

use math::Vec2D;
use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

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
            board: Board::new(Vec2D::new(4, 4)),
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

                let o: Vec2D<f64> = curve.first() * scale + offset;
                ctx.move_to(o.x, o.y);

                let last: Vec2D<f64> = curve.last() * scale + offset;

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

            web_sys::console::clear();

            for node in model.board.graph.nodes() {
                web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", &node)));

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

            ctx.begin_path();

            ctx.move_to(
                offset.x + model.board.arrow.0.x as f64 * scale.x,
                offset.y + model.board.arrow.0.y as f64 * scale.y,
            );

            let angle: f64 = model.board.arrow.1.into();

            ctx.line_to(
                offset.x + model.board.arrow.0.x as f64 * scale.x + 20. * angle.cos(),
                offset.y + model.board.arrow.0.y as f64 * scale.y + 20. * angle.sin(),
            );

            ctx.stroke();

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
                0 => model.board.up_left_1(),
                1 => model.board.up_left_2(),
                2 => model.board.up_left_3(),
                3 => model.board.up_right_3(),
                4 => model.board.up_right_2(),
                5 => model.board.up_right_1(),
                6 => model.board.up_left_1(),
                7 => model.board.up_left_2(),
                8 => model.board.up_left_3(),
                9 => model.board.up_right_3(),
                10 => model.board.up_right_2(),
                11 => model.board.up_right_1(),
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
 */

/* // (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {

}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
} */

use std::collections::BTreeSet;

use backend::Board;
use backend::Tile;
use math::Vec2D;
use seed::{prelude::*, *};

struct PYC {
    board: Board,
    canvas: ElRef<web_sys::HtmlCanvasElement>,
    size: u16,
    scale: f64,
    offset: f64,
}

const CANVAS_SIZE: u16 = 400;

impl PYC {
    fn init(_: Url, orders: &mut impl Orders<Msg>) -> Self {
        orders.after_next_render(|_| Msg::Redraw);

        Self {
            board: Board::empty_start(Vec2D::new(5, 5)),
            canvas: ElRef::default(),
            size: CANVAS_SIZE,
            scale: CANVAS_SIZE as f64 / 11.0,
            offset: CANVAS_SIZE as f64 / 22.0,
        }
    }

    fn update(msg: Msg, model: &mut PYC, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Redraw => {
                model.draw();
            }
            Msg::Tile(tile) => {
                model.board.step(tile);
                orders.send_msg(Msg::Redraw);
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div![
            canvas![
                C!["board"],
                el_ref(&self.canvas),
                attrs!(
                    At::Width => px(self.size),
                    At::Height => px(self.size),
                ),
                /* style!(
                    St::Border => "1px solid black",
                ) */
            ],
            tile_pad(self.board.tiles(), self.size)
        ]
    }

    fn draw(&mut self) {
        let canvas = self.canvas.get().expect("Could not get canvas");
        let ctx = seed::canvas_context_2d(&canvas);

        self.clear(&ctx);
        self.draw_points(&ctx);
        //self.draw_nodes(&ctx);
        self.draw_path(&ctx);
        self.draw_intersections(&ctx);
        self.draw_arrow(&ctx);
    }

    fn clear(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.size as f64, self.size as f64);
    }

    fn draw_points(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        for i in 0_u8..=10 {
            for j in 0_u8..=10 {
                ctx.begin_path();

                ctx.arc(
                    self.offset + self.scale * i as f64,
                    self.offset + self.scale * j as f64,
                    2.,
                    0.,
                    std::f64::consts::TAU,
                )
                .unwrap();

                ctx.fill();
            }
        }
    }

    fn draw_path(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();

        for (_, edge) in self.board.graph().edges() {
            let mut o = edge.first() * self.scale + self.offset;

            for p in edge.path().iter().skip(1) {
                let p = *p * self.scale + self.offset;

                ctx.move_to(o.x, o.y);
                ctx.line_to(p.x, p.y);

                o = p;
            }
        }

        ctx.stroke();
    }

    fn draw_arrow(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let arrow = self.board.arrow();

        let mut position: Vec2D<f64> = (*arrow.position()).into();
        position *= self.scale;
        position += self.offset;

        let direction = Vec2D::from_polar((*arrow.rotation()).into(), self.scale * 0.4) + position;

        ctx.begin_path();

        ctx.move_to(position.x, position.y);
        ctx.line_to(direction.x, direction.y);

        ctx.stroke();
    }

    fn draw_nodes(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        for node in self.board.graph().nodes() {
            ctx.begin_path();

            ctx.arc(
                self.offset + self.scale * node.x as f64,
                self.offset + self.scale * node.y as f64,
                4.,
                0.,
                std::f64::consts::TAU,
            )
            .unwrap();

            ctx.stroke();
        }
    }

    fn draw_intersections(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        for intersection in self.board.intersection_points() {
            ctx.begin_path();

            ctx.arc(
                self.offset + self.scale * intersection.x as f64,
                self.offset + self.scale * intersection.y as f64,
                4.,
                0.,
                std::f64::consts::TAU,
            )
            .unwrap();

            ctx.stroke();
        }
    }
}

fn tile_pad(tiles: &BTreeSet<Tile>, width: u16) -> Node<Msg> {
    div![
        C!["tile-pad"],
        style! {
            St::Width => px(width),
        },
        div![
            tile_button(Tile::up_left_1(), tiles),
            tile_button(Tile::up_left_2(), tiles),
            tile_button(Tile::up_left_3(), tiles),
            tile_button(Tile::up_right_3(), tiles),
            tile_button(Tile::up_right_2(), tiles),
            tile_button(Tile::up_right_1(), tiles),
        ],
        div![
            tile_button(Tile::down_left_1(), tiles),
            tile_button(Tile::down_left_2(), tiles),
            tile_button(Tile::down_left_3(), tiles),
            tile_button(Tile::down_right_3(), tiles),
            tile_button(Tile::down_right_2(), tiles),
            tile_button(Tile::down_right_1(), tiles),
        ],
    ]
}

fn tile_button(tile: Tile, tiles: &BTreeSet<Tile>) -> Node<Msg> {
    button![
        format!("{:?}", &tile),
        IF!(tiles.contains(&tile) => attrs!{At::Disabled => true}),
        ev(Ev::Click, move |_| Msg::Tile(tile))
    ]
}

#[derive(Clone, Copy)]
enum Msg {
    Redraw,
    Tile(Tile),
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", PYC::init, PYC::update, PYC::view);
}
