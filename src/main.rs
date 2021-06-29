use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use legion::*;
use std::cmp::{max, min};

struct State{
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);

        let mut query = <(&Position, &Renderable)>::query();

        for(position, renderable) in query.iter_mut(&mut self.ecs) {
            ctx.set(position.x, position.y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){
    let mut query = <(&Player, &mut Position)>::query();

    for(_player, position) in query.iter_mut(ecs) {
        position.x = min(79, max(0, position.x + delta_x));
        position.y = min(49, max(0, position.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
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
    gs.ecs.push((Position{x: 0, y: 0}, Renderable{glyph: rltk::to_cp437('@'), fg: RGB::named(rltk::YELLOW), bg: RGB::named(rltk::BLACK)}, Player{ }));
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Player {}