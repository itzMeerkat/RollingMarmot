use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::time::Duration;

const ARENA_WIDTH: i32 = 32;
const ARENA_HEIGHT: i32 = 32;
const AGENT_COUNT: u32 = 4;
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

struct Agent;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());

}

fn game_setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let xs = [0,5,10,15];
    for i in 0..AGENT_COUNT {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            })
            .with(Agent)
            .with(Position{
                x: xs[i as usize],
                y: xs[i as usize]})
            .with(Size::square(0.9));
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    for (size, mut sprite) in q.iter_mut() {
        let window = windows.get_primary().unwrap();
        sprite.size = Vec2::new(
            size.width as f32 / ARENA_WIDTH as f32 * window.width() as f32,
            size.height as f32 / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(p: f32, bound_window: f32, bound_game: f32) -> f32 {
        p / bound_game * bound_window - (bound_window / 2.)
    }
    let window = windows.get_primary().unwrap();
    
    for (pos, mut transform) in q.iter_mut() {
        //println!("{} {}", pos.x, pos.y);
        let new_vec = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
        let diff = new_vec - transform.translation;
        transform.translation+=diff;
    }
}

struct MoveEvent {
    sender: Entity,
    to: Position
}

fn ticker_system(time: Res<Time>, mut timer: ResMut<AgentMoveTimer>) {
    timer.0.tick(time.delta_seconds);
}

fn rnd_agent(timer: ResMut<AgentMoveTimer>, mut events: ResMut<Events<MoveEvent>>, rndagents: Query<(Entity, &Position)>) {
    if !timer.0.finished {
        return;
    }
 
    for (e, pos) in rndagents.iter() {
        let v: i32 = rand::random::<i32>() % 4;
        let mut new_ev = MoveEvent{sender:e, to: pos.clone()};
        if v == 0 {
            new_ev.to.x+=1;
        }
        if v == 1 {
            new_ev.to.x-=1;
        }
        if v == 2 {
            new_ev.to.y+=1;
        }
        if v == 3 {
            new_ev.to.y-=1;
        }
        events.send(new_ev);
    }
}

fn move_handler(events: ResMut<Events<MoveEvent>>, mut reader: Local<EventReader<MoveEvent>>, mut agents: Query<(Entity, &mut Position)>) {
    for e in reader.iter(&events) {
        if let Ok((_,mut c))=agents.get_mut(e.sender) {
            if e.to.x <0 || e.to.x >= ARENA_HEIGHT || e.to.y<0 || e.to.y>= ARENA_WIDTH {
                println!("Move action denied");
            } else {
                c.x = e.to.x;
                c.y = e.to.y;
            }
        }
    }
}
struct AgentMoveTimer(Timer);

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 800,
            height: 800,
            ..Default::default()
        })
        .add_resource(AgentMoveTimer(Timer::new(Duration::from_millis(500. as u64), true)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_event::<MoveEvent>()
        .add_stage("Game")
        .add_stage_after("Game", "Sum")
        .add_system_to_stage("Game", rnd_agent.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(ticker_system.system())
        .add_system_to_stage("Sum", move_handler.system()) 
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}