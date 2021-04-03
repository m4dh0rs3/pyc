use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use backend::prelude::*;
use math::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Polycentrics>::new().mount_to_body();
}

/// Holds game and UI state.
struct Polycentrics {
    link: ComponentLink<Self>,
    board: Board,
    board_canvas: BoardCanvas,
    tile_pad: TilePad,
}

/// Draws the [`Board`] on a canvas.
struct BoardCanvas {
    node_ref: NodeRef,
    size: u16,
    scale: f64,
    offset: f64,
}

struct TilePad {}

enum Msg {
    DrawBoardCanas,
    SetTile(usize),
}

#[derive(Clone, Properties)]
struct GameProps {
    points_num: u8,
    canvas_size: u16,
}

impl Default for GameProps {
    fn default() -> Self {
        Self {
            points_num: 11,
            canvas_size: 400,
        }
    }
}

impl Component for Polycentrics {
    type Message = Msg;
    type Properties = GameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::DrawBoardCanas);

        Self {
            link,
            board: Board::new(
                Player::Gamma,
                Arrow::new(
                    Vec2D::new(props.points_num as i8 / 2, props.points_num as i8 / 2),
                    0.0.into(),
                ),
                Curve::convex_4x3(),
                props.points_num,
            ),
            board_canvas: BoardCanvas {
                node_ref: NodeRef::default(),
                size: props.canvas_size,
                scale: props.canvas_size as f64 / props.points_num as f64,
                offset: props.canvas_size as f64 / props.points_num as f64 / 2.0,
            },
            tile_pad: TilePad {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DrawBoardCanas => {
                self.board_canvas.draw(&self.board);

                true
            }
            Msg::SetTile(tile) => {
                self.board.step(tile);

                self.link.send_message(Msg::DrawBoardCanas);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="polycentrics">
                { self.board_canvas.view() }
                { self.tile_pad.view(&self.link, self.board.options()) }
            </div>
        }
    }
}

impl TilePad {
    fn view(&self, link: &ComponentLink<Polycentrics>, tiles: &Vec<Curve>) -> Html {
        html! {
            <div class="tile-pad">
                { tiles.iter().enumerate().map(|(i, tile)| Self::tile_button_view(link, format!("{:?}", tile), i)).collect::<Html>() }
            </div>
        }
    }

    fn tile_button_view(link: &ComponentLink<Polycentrics>, name: String, tile: usize) -> Html {
        html! {
            <button onclick=link.callback(move |_| Msg::SetTile(tile))>{ name }</button>
        }
    }
}

impl BoardCanvas {
    fn view(&self) -> Html {
        html! {
            <canvas class="board-canvas" ref=self.node_ref.clone() width=self.size height=self.size />
        }
    }

    fn draw(&self, board: &Board) {
        let ctx = self.get_ctx();

        self.clear(&ctx);
        self.draw_points(&ctx, board);
        self.draw_path(&ctx, board);
        self.draw_nodes(&ctx, board);
        self.draw_arrow(&ctx, board);
    }

    fn get_ctx(&self) -> CanvasRenderingContext2d {
        Into::<CanvasRenderingContext2d>::into(Into::<JsValue>::into(
            self.node_ref
                .cast::<HtmlCanvasElement>()
                .expect("Could not get canvas")
                .get_context("2d")
                .expect("Could not get 2d context from canvas")
                .expect("Could not get JS value from context"),
        ))
    }

    fn clear(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.size as f64, self.size as f64);
    }

    fn draw_points(&self, ctx: &CanvasRenderingContext2d, board: &Board) {
        ctx.set_stroke_style(&JsValue::from_str("black"));

        for i in 0..board.get_size() {
            for j in 0..board.get_size() {
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

    fn draw_path(&self, ctx: &web_sys::CanvasRenderingContext2d, board: &Board) {
        ctx.set_line_width(self.scale / 16.0);

        for (_, edge) in board.get_edges() {
            ctx.begin_path();

            let mid = edge.get_mid();
            let (start, end) = match edge.get_turn() {
                Turn::Positive => (edge.get_start(), edge.get_end()),
                Turn::Negative => (edge.get_end(), edge.get_start()),
            };

            ctx.arc(
                self.offset + mid.x as f64 * self.scale,
                self.offset + mid.y as f64 * self.scale,
                edge.get_radius() as f64 * self.scale,
                *start,
                *end,
            );

            ctx.stroke();
        }

        ctx.set_stroke_style(&JsValue::from_str("black"));
    }

    fn draw_arrow(&self, ctx: &web_sys::CanvasRenderingContext2d, board: &Board) {
        let mut position: Vec2D<f64> = board.get_position().into();
        position *= self.scale;
        position += self.offset;

        let direction = Vec2D::from_polar(board.get_angle(), self.scale * 0.4) + position;

        ctx.begin_path();

        ctx.set_line_width(self.scale / 16.0);
        ctx.set_stroke_style(&JsValue::from_str("black"));

        ctx.move_to(position.x, position.y);
        ctx.line_to(direction.x, direction.y);

        ctx.stroke();
    }

    fn draw_nodes(&self, ctx: &web_sys::CanvasRenderingContext2d, board: &Board) {
        ctx.set_stroke_style(&JsValue::from_str("red"));
        ctx.set_line_width(self.scale / 36.0);

        for node in board.get_nodes() {
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
