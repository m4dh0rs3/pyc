#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use backend::{Board, Tile};
use math::Vec2D;

use std::collections::BTreeSet;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Game>::new().mount_to_body();
}

pub(crate) struct Game {
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    board: backend::Board,
    canvas_size: u16,
    point_scale: f64,
    point_offset: f64,
}

#[derive(Properties, Clone, PartialEq)]
struct GameProps {
    points_num: u8,
    canvas_size: u16,
    curve_res: usize,
}

impl Default for GameProps {
    fn default() -> Self {
        Self {
            points_num: 11,
            canvas_size: 400,
            curve_res: 12,
        }
    }
}

pub(crate) enum Msg {
    Tile(Tile),
    Draw,
}

impl Component for Game {
    type Message = Msg;
    type Properties = GameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            canvas_ref: NodeRef::default(),
            board: Board::empty_start(
                Vec2D::new((props.points_num / 2) as i8, (props.points_num / 2) as i8),
                11,
                12,
            ),
            canvas_size: props.canvas_size,
            point_scale: props.canvas_size as f64 / props.points_num as f64,
            point_offset: props.canvas_size as f64 / props.points_num as f64 / 2.0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tile(tile) => {
                self.board.step(tile);
                self.link.send_message(Msg::Draw);
                false
            }
            Msg::Draw => {
                let ctx = self.get_ctx();
                self.clear(&ctx);
                self.draw_points(&ctx);

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <canvas class="board" ref=self.canvas_ref.clone() width=self.canvas_size height=self.canvas_size style="border: 5px solid black;" />
                { self.tile_pad_view() }
            </div>
        }
    }
}

impl Game {
    fn tile_button_view(&self, tile: Tile) -> Html {
        html! {
            <button onclick=self.link.callback(move |_| Msg::Tile(tile))>{ format!("{:?}", tile) }</button>
        }
    }

    fn tile_pad_view(&self) -> Html {
        html! {
            <div class="tile_pad">
                <div>
                    { self.tile_button_view(Tile::up_left_1()) }
                    { self.tile_button_view(Tile::up_left_2()) }
                    { self.tile_button_view(Tile::up_left_3()) }

                    { self.tile_button_view(Tile::up_right_3()) }
                    { self.tile_button_view(Tile::up_right_2()) }
                    { self.tile_button_view(Tile::up_right_1()) }
                </div>
                <div>
                    { self.tile_button_view(Tile::down_left_1()) }
                    { self.tile_button_view(Tile::down_left_2()) }
                    { self.tile_button_view(Tile::down_left_3()) }

                    { self.tile_button_view(Tile::down_right_3()) }
                    { self.tile_button_view(Tile::down_right_2()) }
                    { self.tile_button_view(Tile::down_right_1()) }
                </div>
            </div>
        }
    }
}

impl Game {
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

    fn clear(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.canvas_size as f64, self.canvas_size as f64);
    }

    fn draw_points(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("black"));

        for i in 0..self.board.points_num {
            for j in 0..self.board.points_num {
                ctx.begin_path();

                ctx.arc(
                    self.point_offset + self.point_scale * i as f64,
                    self.point_offset + self.point_scale * j as f64,
                    self.point_scale / 16.0,
                    0.,
                    std::f64::consts::TAU,
                )
                .unwrap();

                ctx.fill();
            }
        }
    }
}
