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
            tile_pad(&self.board.tiles, self.size),
            format!("#Tiles: {}", self.board.graph.edges.len()),
            format!("#Nodes: {}", self.board.graph.nodes.len()),
        ]
    }

    fn draw(&mut self) {
        let ctx = self.get_ctx();

        ctx.set_line_join("round");
        ctx.set_line_cap("round");

        self.clear(&ctx);
        self.draw_points(&ctx);
        self.draw_nodes(&ctx);
        self.draw_debug_path(&ctx);
        self.draw_arrow(&ctx);
    }

    fn get_ctx(&self) -> web_sys::CanvasRenderingContext2d {
        seed::canvas_context_2d(&self.canvas.get().expect("Could not get canvas!"))
    }

    fn clear(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.size as f64, self.size as f64);
    }

    fn draw_points(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("black"));

        for i in 0_u8..=10 {
            for j in 0_u8..=10 {
                ctx.begin_path();

                ctx.arc(
                    self.offset + self.scale * i as f64,
                    self.offset + self.scale * j as f64,
                    self.scale / 16.0,
                    0.,
                    std::f64::consts::TAU,
                )
                .unwrap();

                ctx.fill();
            }
        }
    }

    fn draw_path(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_line_width(self.scale / 16.0);
        ctx.begin_path();

        for (_, edge) in &self.board.graph.edges {
            let mut o = edge.first() * self.scale + self.offset;

            for p in edge.path.iter().skip(1) {
                let p = *p * self.scale + self.offset;

                ctx.move_to(o.x, o.y);
                ctx.line_to(p.x, p.y);

                o = p;
            }
        }

        ctx.set_stroke_style(&JsValue::from_str("black"));
        ctx.stroke();
    }

    fn draw_debug_path(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_line_width(self.scale / 16.0);

        for (i, (_, edge)) in self.board.graph.edges.iter().enumerate() {
            let mut o = edge.first() * self.scale + self.offset;

            for (j, p) in edge.path.iter().skip(1).enumerate() {
                let p = *p * self.scale + self.offset;

                ctx.begin_path();

                ctx.move_to(o.x, o.y);
                ctx.line_to(p.x, p.y);

                ctx.set_stroke_style(&JsValue::from_str(&format!(
                    "hsl({}, {}%, {}%)",
                    i * 45 + j * 2,
                    100,
                    50,
                )));

                ctx.stroke();

                o = p;
            }
        }
    }

    fn draw_arrow(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let arrow = self.board.arrow;

        let mut position: Vec2D<f64> = arrow.position.into();
        position *= self.scale;
        position += self.offset;

        let direction = Vec2D::from_polar(arrow.rotation.into(), self.scale * 0.4) + position;

        ctx.begin_path();

        ctx.set_line_width(self.scale / 16.0);
        ctx.set_stroke_style(&JsValue::from_str("black"));

        ctx.move_to(position.x, position.y);
        ctx.line_to(direction.x, direction.y);

        ctx.stroke();
    }

    fn draw_nodes(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("red"));
        ctx.set_line_width(self.scale / 36.0);

        for node in &self.board.graph.nodes {
            ctx.begin_path();

            ctx.arc(
                self.offset + self.scale * node.x as f64,
                self.offset + self.scale * node.y as f64,
                self.scale / 7.0,
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
