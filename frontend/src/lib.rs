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
                { self.board_canvas_view() }
                { self.tile_pad_view() }
            </div>
        }
    }
}

impl Polycentrics {
    /// View of the board canvas.
    fn board_canvas_view(&self) -> Html {
        html! {
            <canvas class="board-canvas" ref=self.canvas_ref.clone() width=self.canvas_size height=self.canvas_size />
        }
    }

    /// View of the tile pad.
    fn tile_pad_view(&self) -> Html {
        html! {
            <div class="tile-pad">
                { self.board.options().iter().enumerate().map(|(i, tile)| self.tile_button_view(format!("{:?}", tile), i)).collect::<Html>() }
            </div>
        }
    }
}

/* impl Polycentrics {
    /// Debug tiles and nodes view.
    fn tiles_nodes_view(&self) -> Html {
        html! {
            <div>
                { format!("Tiles: {}", self.board.get_edges().len()) }
                { format!("Nodes: {}", self.board.get_nodes().len()) }
                { format!("Polys: {}", self.board.get_polys().len()) }
            </div>
        }
    }

    /// View of the tile pad.
    fn tile_pad_view(&self) -> Html {
        html! {
            <div class="tile-pad">
                { self.board.options().iter().enumerate().map(|(i, tile)| self.tile_button_view(format!("{:?}", tile), i)).collect::<Html>() }
            </div>
        }
    }

    /// A tile button.
    fn tile_button_view(&self, name: String, tile: usize) -> Html {
        html! {
            <button onclick=self.link.callback(move |_| Msg::SetTile(tile))>{ name }</button>
        }
    }

    /// View of the board canvas.
    fn board_canvas_view(&self) -> Html {
        html! {
            <canvas class="board-canvas" ref=self.canvas_ref.clone() width=self.canvas_size height=self.canvas_size />
        }
    }

    /// Redraws the board canavas.
    fn draw(&self) {
        let ctx = self.get_ctx();

        self.clear(&ctx);
        self.draw_points(&ctx);
        self.draw_path(&ctx);
        self.draw_polys(&ctx);
        self.draw_nodes(&ctx);
        self.draw_arrow(&ctx);
    }

    /// Returns the context of the canvas.
    fn get_ctx(&self) -> CanvasRenderingContext2d {
        Into::<CanvasRenderingContext2d>::into(Into::<JsValue>::into(
            self.canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("Could not get canvas")
                .get_context("2d")
                .expect("Could not get 2d context from canvas")
                .expect("Could not get JS value from context"),
        ))
    }

    /// Clears the canvas.
    fn clear(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.canvas_size as f64, self.canvas_size as f64);
    }

    /// Draws the points of the canvas.
    fn draw_points(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("black"));

        for i in 0..self.board.get_size() as usize {
            for j in 0..self.board.get_size() as usize {
                ctx.begin_path();

                match self.board.get_point(i, j) {
                    Some(player) => match player {
                        Player::Gamma => ctx.set_stroke_style(&JsValue::from_str("red")),
                        Player::Delta => ctx.set_stroke_style(&JsValue::from_str("blue")),
                    },
                    None => ctx.set_stroke_style(&JsValue::from_str("black")),
                }

                ctx.arc(
                    self.canvas_offset + self.canvas_scale * i as f64,
                    self.canvas_offset + self.canvas_scale * j as f64,
                    self.canvas_scale / 16.0,
                    0.,
                    std::f64::consts::TAU,
                )
                .unwrap();

                ctx.fill();
            }
        }
    }

    /// Draw the path of tiles on the canvas.
    fn draw_path(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_line_width(self.canvas_scale / 16.0);

        for (_, edge) in self.board.get_edges() {
            ctx.begin_path();

            let mid = edge.get_mid();

            let (start, end) = match edge.get_turn() {
                Turn::Positive => (edge.get_start(), edge.get_end()),
                Turn::Negative => (edge.get_end(), edge.get_start()),
            };

            ctx.arc(
                self.canvas_offset + mid.x as f64 * self.canvas_scale,
                self.canvas_offset + mid.y as f64 * self.canvas_scale,
                edge.get_radius() as f64 * self.canvas_scale,
                *start,
                *end,
            )
            .unwrap();

            ctx.stroke();
        }

        ctx.set_stroke_style(&JsValue::from_str("black"));
    }

    fn draw_polys(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("red"));
        ctx.begin_path();

        for poly in self.board.get_polys() {
            if poly.len() > 1 {
                let mut first = poly.first().unwrap();

                for second in poly.iter().skip(1).chain(vec![*first].iter()) {
                    let first_point = self.board.get_nodes()[*first];
                    let second_point = self.board.get_nodes()[*second];

                    ctx.move_to(
                        first_point.x * self.canvas_scale + self.canvas_offset,
                        first_point.y * self.canvas_scale + self.canvas_offset,
                    );

                    ctx.line_to(
                        second_point.x * self.canvas_scale + self.canvas_offset,
                        second_point.y * self.canvas_scale + self.canvas_offset,
                    );

                    first = second;
                }
            }
        }

        ctx.stroke();
    }

    /// Draw the arrow on to the canvas.
    fn draw_arrow(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let mut position: Vec2D<f64> = self.board.get_position().into();
        position *= self.canvas_scale;
        position += self.canvas_offset;

        let direction =
            Vec2D::from_polar(self.board.get_angle(), self.canvas_scale * 0.4) + position;

        ctx.begin_path();

        ctx.set_line_width(self.canvas_scale / 16.0);
        ctx.set_stroke_style(&JsValue::from_str("black"));

        ctx.move_to(position.x, position.y);
        ctx.line_to(direction.x, direction.y);

        ctx.stroke();
    }

    /// Draw the nodes on the canvas.
    fn draw_nodes(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("red"));
        ctx.set_line_width(self.canvas_scale / 36.0);

        for node in self.board.get_nodes() {
            ctx.begin_path();

            ctx.arc(
                self.canvas_offset + self.canvas_scale * node.x as f64,
                self.canvas_offset + self.canvas_scale * node.y as f64,
                self.canvas_scale / 7.0,
                0.,
                std::f64::consts::TAU,
            )
            .unwrap();

            ctx.stroke();
        }
    }
}
 */
