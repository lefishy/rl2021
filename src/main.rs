use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use legion::*;
use std::cmp::{max, min};

struct State{
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        let mut query = <(&Position, &Renderable)>::query();

        for(position, renderable) in query.iter_mut(&mut self.ecs) {
            ctx.set(position.x, position.y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::default()
    };
    gs.ecs.push((Position{x: 0, y: 0}, Renderable{glyph: rltk::to_cp437('@'), fg: RGB::named(rltk::YELLOW), bg: RGB::named(rltk::BLACK)}));
    rltk::main_loop(context, gs)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position{
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Renderable{
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}