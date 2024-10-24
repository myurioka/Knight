mod knight;
mod parwn;
mod ornament;
mod bishop;
mod shot;
mod fire;
mod rook;
use rand::prelude::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use knight::knight::*;
use parwn::parwn::*;
use bishop::bishop::*;
use shot::shot::*;
use fire::fire::*;
use ornament::ornament::*;
use rook::rook::*;
use wasm_bindgen::JsCast;
use crate::{
    browser,
    engine::{Point, Game, KeyState, Rect, Renderer, FONT_L, FONT_S, FONT_CENTER, FONT_LEFT}
};
use web_sys::HtmlInputElement;

/* <-- CONSTANT VALUE */
const FLOOR_HEIGHT: i16 = 280;
const STAGE_LEFT:i16 = 100;
const STAGE_RIGHT:i16 = 1200;
const KNIGHT_X:i16 = STAGE_LEFT + 20;
const RUNNING_SPEED:i16 = 1;
const ORNAMENT_X:i16 = STAGE_LEFT + 20;
const ORNAMENT_Y:i16 = 400;
const PARWN_SPEED:i16 = 1;
const PARWN_WIDTH:i16 = 80;
const PARWN_HEIGHT:i16 = 30;
const PARWN1_X:i16 = 550;
const PARWN2_X:i16 = 750;
const BISHOP_X:i16 = 540;
const BISHOP_Y:i16 = 120;
const BISHOP_MOVE: i16 = 100;
const BISHOP_WIDTH: i16 = 30;
const BISHOP_HEIGHT: i16 = 70;
const BISHOP_TERM:i16 = 80;
const ROOK_X:i16 = STAGE_RIGHT + 50;
const ROOK_Y:i16 = 220;
const ROOK_WIDTH: i16 = 300;
const ROOK_TERM: i16 = 230;
const ROOK_HEIGHT: i16 = 180;
const SHOT_SPEED:i16 = 1;
const SHOT_WIDTH: i16 = 50;
const SHOT_HEIGHT: i16 = 90;
const FIRE_TERM:i16 = 200;
const FIRE_SPEED:i16 = 1;
const FIRE_WIDTH: i16 = 40;
const FIRE_HEIGHT: i16 = 40;
const MESSAGE_X: i16 = 20;
const MESSAGE_Y: i16 = 380;
const MESSAGE_WIDTH: i16 = 2;
const MESSAGE_HEIGHT: i16 = 100;
const MESSAGE_TIME: i16 =100;
const MESSAGE_OPENING: [&str; 3] = ["Welcome to Basic World !!","In this world, imagination is the most important thing.",""];
const MESSAGE_RUNNING: [&str; 3] = ["Use your imagination to the fullest and clear the world.","Here we go!!",""];
const MESSAGE_GAMEOVER: [&str; 3] = ["Thank you for your playing.","",""];
const BRANK: [&str; 3] = ["", "", ""];
/* CONSTANT VALUE --> */

pub struct GameStage {
    machine: Option<GameStageStateMachine>,
}
impl GameStage {
    pub fn new() -> Self {
        GameStage { machine: None }
    }
}
enum GameStageStateMachine {
    Ready(GameStageState<Ready>),
    Playing(GameStageState<Playing>),
    GameOver(GameStageState<GameOver>),
}
impl GameStageStateMachine {
    fn new(material: Material) -> Self {
        GameStageStateMachine::Ready(GameStageState::new(material))
    }
    fn update(self, _keystate: &KeyState) -> Self {
        match self {
            GameStageStateMachine::Ready(state) => state.update(_keystate).into(),
            GameStageStateMachine::Playing(state) => state.update(_keystate).into(),
            GameStageStateMachine::GameOver(state) => state.update(_keystate).into(),
        }
    }
    fn draw(&self, renderer: &Renderer) {
        match self {
            GameStageStateMachine::Ready(state) => state.draw(renderer),
            GameStageStateMachine::Playing(state) => state.draw(renderer),
            GameStageStateMachine::GameOver(state) => state.draw(renderer),
        };
    }
}
impl From<GameStageState<Ready>> for GameStageStateMachine {
    fn from(state: GameStageState<Ready>) -> Self {
        GameStageStateMachine::Ready(state)
    }
}
impl From<GameStageState<Playing>> for GameStageStateMachine {
    fn from(state: GameStageState<Playing>) -> Self {
        GameStageStateMachine::Playing(state)
    }
}
impl From<GameStageState<GameOver>> for GameStageStateMachine {
    fn from(state: GameStageState<GameOver>) -> Self {
        GameStageStateMachine::GameOver(state)
    }
}

struct GameStageState<T> {
    _state: T,
    material: Material,
}
impl<T> GameStageState<T> {
    fn draw(&self, renderer: &Renderer) {
        self.material.draw(renderer);
    }
}
struct Ready;
impl GameStageState<Ready> {
    fn new(material: Material) -> GameStageState<Ready> {
        GameStageState { _state: Ready, material,}
    }
    fn start_running(self) -> GameStageState<Playing> {
        GameStageState { _state: Playing, material: self.material,}
    }
    fn update(self, _keystate: &KeyState) -> ReadyEndState {
        if _keystate.is_pressed("Space") {
            return ReadyEndState::Complete(self.start_running());
        }
        ReadyEndState::Continue(self)
    }
}
enum ReadyEndState {
    Complete(GameStageState<Playing>),
    Continue(GameStageState<Ready>),
}
impl From<ReadyEndState> for GameStageStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(running) => running.into(),
            ReadyEndState::Continue(ready) => ready.into(),
        }
    }
}

struct Playing;
impl GameStageState<Playing> {
    fn update(mut self, _keystate: &KeyState) -> RunningEndState {

        self.material.frame +=1;
        let mut _current_velocity:Point = self.material.knight.velocity();

        // current_velocity
        let mut _current_velocity:Point = self.material.knight.velocity();

        // knight reach left Edge
        if self.material.ornament.position().x  > KNIGHT_X {
            self.material.knight.run(Point{x:0, y:0});
            self.material.ornament.run(Point{x:0, y:0});
            self.material.parwns.iter_mut().for_each(|parwn| {
                parwn.run(Point{x: -PARWN_SPEED, y:0});
            });
            self.material.shots.iter_mut().for_each(|shot| {
                shot.run(Point{x:0, y:SHOT_SPEED});
            });
            self.material.fires.iter_mut().for_each(|fire| {
                fire.run(Point{x: -FIRE_SPEED, y:0});
            });
            self.material.rooks.iter_mut().for_each(|rook| {
                rook.run(Point{x:0, y: 0});
            });
        }

        // knight reach goal
        if self.material.ornament.position().x < -STAGE_RIGHT {
            self.material.ornament.run(Point{x:0, y:0});
            self.material.parwns.iter_mut().for_each(|parwn| {
                parwn.run(Point{x: 0, y:0});
            });
            self.material.shots.iter_mut().for_each(|shot| {
                shot.run(Point{x:0, y:SHOT_SPEED});
            });
            self.material.knight.clear();
            return RunningEndState::Complete(
                GameStageState {
                    _state: GameOver,
                    material: self.material,
                }
            );
        }
        
        if _keystate.is_pressed("ArrowLeft") {
            self.material.knight.run(Point{x: -RUNNING_SPEED, y:0});
            _current_velocity = self.material.knight.velocity();
        }
        if _keystate.is_pressed("ArrowRight") {
            self.material.knight.run(Point{x: RUNNING_SPEED, y:0});
            _current_velocity = self.material.knight.velocity();
        }
        if _keystate.is_pressed("ArrowUp") {
            self.material.knight.jump();
        }
        if _keystate.is_pressed("Space") {
            self.material.knight.attack();
            self.material.parwns.retain(|parwn| !self.material.knight.attacking_box().intersects(&parwn.bounding_box(PARWN_WIDTH, PARWN_HEIGHT)));
            self.material.rooks.retain(|rook| !self.material.knight.attacking_box().intersects(&rook.bounding_box(ROOK_WIDTH, ROOK_HEIGHT)));
            self.material.bishops.iter_mut().for_each(|bishop|{
                if self.material.knight.attacking_box().intersects(&bishop.bounding_box(BISHOP_WIDTH, BISHOP_HEIGHT)) {
                    self.material.count_bishops = 0;
                }
            });
        }

        // Parwns & Ornament
        self.material.ornament.run(Point{x: - _current_velocity.x, y:0});
        self.material.parwns.iter_mut().for_each(|parwn| {
            parwn.run(Point{x: -PARWN_SPEED - _current_velocity.x, y:0});
        });

        // Bishop & Shot
        self.material.shots.retain(|shot| shot.position().y < FLOOR_HEIGHT);
  
        if self.material.count_bishops == 1 && self.material.frame % BISHOP_TERM == 0 {
            if self.material.bishops.len() == 1 {
                self.material.bishops.remove(0);
            } else { 
                let _x = thread_rng().gen_range(0..5) * BISHOP_MOVE;
                self.material.bishops.push(
                    Bishop::new(
                        Point { x: _x, y: BISHOP_Y},
                        Point { x: - _current_velocity.x, y: 0}
                    )
                );
/*
                self.material.shots.push(
                    Shot::new(
                        Point { x: _x, y: BISHOP_Y - 20},
                        Point { x: _current_velocity.x, y: SHOT_SPEED }
                    )
                );
*/
            }
        }
        self.material.bishops.iter_mut().for_each(|bishop| {
            bishop.run(Point{x: -_current_velocity.x, y: 0});
        });
        self.material.shots.iter_mut().for_each(|shot| {
            shot.run(Point{x: -_current_velocity.x, y: SHOT_SPEED});
        });

        // Rook & Fire
        if self.material.rooks.len() > 0 && self.material.frame % ROOK_TERM == 0 && self.material.fires.len() == 0 {
            /*
            self.material.fires.push(
                Fire::new(
                    Point { x: self.material.rooks[0].position().x, y: self.material.rooks[0].position().y+40},
                    Point { x: - _current_velocity.x, y: 0 }
                )
            );
            */
        }
        self.material.rooks.iter_mut().for_each(|rook| {
            rook.run(Point{x: -_current_velocity.x, y: 0});
        });
        self.material.fires.iter_mut().for_each(|fire| {
            fire.run(Point{x: -_current_velocity.x - FIRE_SPEED, y: 0});
        });
        // Fire Retain
        self.material.fires.retain(|fire| fire.position().x > STAGE_LEFT);

        // check Knight collision
        let mut knocked = false;
        self.material.parwns.iter_mut().for_each(|parwn| {
            if self.material.knight.bounding_box().intersects(&parwn.bounding_box(PARWN_WIDTH, PARWN_HEIGHT)){
                knocked = true;
            }
        });
        self.material.shots.iter_mut().for_each(|shot| {
            if self.material.knight.bounding_box().intersects(&shot.bounding_box(SHOT_WIDTH, SHOT_HEIGHT)){
                knocked = true;
            }
        });
        self.material.fires.iter_mut().for_each(|fire| {
            if self.material.knight.bounding_box().intersects(&fire.bounding_box(FIRE_WIDTH, FIRE_HEIGHT)){
                knocked = true;
            }
        });
        self.material.rooks.iter_mut().for_each(|rook| {
            if self.material.knight.bounding_box().intersects(&rook.bounding_box(ROOK_WIDTH, ROOK_HEIGHT)){
                knocked = true;
            }
        });
        if knocked {
            self.material.knight.knocked();
            return RunningEndState::Complete(
                GameStageState {
                    _state: GameOver,
                    material: self.material,
                }
            );
        }

        // Update each of Material
        self.material.parwns.iter_mut().for_each(|parwn| {
            parwn.update();
        });
        self.material.shots.iter_mut().for_each(|shot| {
            shot.update();
        });
        self.material.fires.iter_mut().for_each(|fire| {
            fire.update();
        });
        self.material.rooks.iter_mut().for_each(|rook| {
            rook.update();
        });
        self.material.knight.update();
        self.material.ornament.update();

        RunningEndState::Continue(self)
    }
}
impl From<RunningEndState> for GameStageStateMachine {
    fn from(state: RunningEndState) -> Self {
        match state {
            RunningEndState::Continue(running) => running.into(),
            RunningEndState::Complete(gameover) => gameover.into(),
        }
    }
}
struct GameOver;
impl GameStageState<GameOver> {
    fn update(self, _keystate: &KeyState) -> GameOverEndState {
        if _keystate.is_pressed("Space") {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Ready>{
        GameStageState {
            _state: Ready,
            material: Material::reset(self.material)
        }
    }
}
enum RunningEndState {
    Continue(GameStageState<Playing>),
    Complete(GameStageState<GameOver>),
}

enum GameOverEndState {
    Complete(GameStageState<Ready>),
    Continue(GameStageState<GameOver>),
}
impl From<GameOverEndState> for GameStageStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Continue(running) => running.into(),
            GameOverEndState::Complete(ready) => ready.into(),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Context {
    pub position: Point,
    pub velocity: Point,
}
impl Context {
    fn update(self) -> Self {
        self
    }
    fn run(mut self, velocity: Point) -> Self {
        self.velocity = velocity;
        self
    }
}

#[derive(Copy, Clone)]
pub struct State<S> {
    pub context: Context,
    _state: S,
}
impl<S> State<S> {
    pub fn context(&self) -> &Context {
        &self.context
    }
    fn update_context(&mut self){
        self.context = self.context.update();
    }
}
pub enum Event {
    Run(Point),
    Update,
}

#[derive(Copy, Clone)]
pub enum StateMachine{
    Running(State<Running>),
}
#[derive(Copy, Clone)]
pub struct Running;
impl State<Running> {
    pub fn new(position: Point, velocity: Point) -> Self {
        State {
            context: Context{
                position: position,
                velocity: velocity,
            },
            _state: Running {},
        }
    }
    pub fn update(mut self)  -> State<Running> {
            self.context.position.x = self.context.position.x + self.context.velocity.x;
            self.context.position.y = self.context.position.y + self.context.velocity.y;
            self.update_context();
            self
        }
        pub fn run(self, velocity:Point) -> State<Running> {
            State {
                context: self.context.run(velocity),
                _state: Running{},
        }
        }
    }
impl StateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (StateMachine::Running(state), Event::Run(velocity)) => state.run(velocity).into(),
            (StateMachine::Running(state), Event::Update) => state.update().into(),
        }
    }
    pub fn context(&self) -> &Context {
        match self {
            StateMachine::Running(state) => state.context(),
        }
    }
    fn update(self) -> Self {
        self.transition(Event::Update)
    }
}
impl From<State<Running>> for StateMachine{
    fn from(state: State<Running>) -> Self {
        StateMachine::Running(state)
    }
}

pub trait Piece {
    fn new(postion: Point, velocity: Point) -> Self;
    fn get_state_machine(&self) -> StateMachine;
    fn set_state_machine(&mut self, state_machine:StateMachine);
    fn update(&mut self){
        let _state_machine = self.get_state_machine();
        self.set_state_machine(_state_machine);
    }
    fn run(&mut self, velocity:Point){
        let _from_state_machine = self.get_state_machine();
        let _to_state_machine = _from_state_machine.transition(Event::Run(velocity));
        self.set_state_machine(_to_state_machine);
    }
    fn bounding_box(&self,width:i16, height:i16) -> Rect {
        let _state_machine:StateMachine = self.get_state_machine();
        let _x = _state_machine.context().position.x;
        let _y = _state_machine.context().position.y;
        Rect {
            x: _x,
            y: _y,
            width: width,
            height: height,
            //character: BRANK,
            character: ["*****","******","******"],
            font_size: FONT_L,
            font_align: FONT_CENTER,
        }
    }
    fn position(&self) -> Point{
        let _state_machine = self.get_state_machine();
        _state_machine.context().position
    }
    fn draw(&self, renderer: &Renderer);
}

pub struct Material {
    frame: i16,
    count_bishops: usize,
    knight: Knight,
    parwns: Vec<Parwn>,
    bishops: Vec<Bishop>,
    shots: Vec<Shot>,
    fires: Vec<Fire>,
    rooks: Vec<Rook>,
    ornament: Ornament,
}
impl Material {
    fn new()->Self { 
        Material {
            frame: 10,
            count_bishops: 1,
            knight: Knight::new(
                Point { x: KNIGHT_X, y: FLOOR_HEIGHT },
                Point { x: RUNNING_SPEED, y: 0 }
            ),
            parwns: vec![
            /*
                Parwn::new(
                    Point { x: PARWN1_X, y: FLOOR_HEIGHT },
                    Point { x: -RUNNING_SPEED - PARWN_SPEED, y: 0 }
                ),
                Parwn::new(
                    Point { x: PARWN2_X, y: FLOOR_HEIGHT },
                    Point { x: -RUNNING_SPEED - PARWN_SPEED, y: 0 }
                ),
            */
            ],
            bishops: vec![
                Bishop::new(
                    Point { x: BISHOP_X, y: BISHOP_Y},
                    Point { x: -RUNNING_SPEED, y: 0}
                ),
            ],
            shots: vec![],
            fires: vec![],
            rooks: vec![
                Rook::new(
                    Point { x: ROOK_X, y: ROOK_Y},
                    Point { x: -RUNNING_SPEED, y: 0 }
                ),
            ],
            ornament: Ornament::new(
                Point { x: ORNAMENT_X, y: ORNAMENT_Y },
                Point { x: -RUNNING_SPEED, y: 0 }
            ),
        }
    }
    fn reset(material: Self) -> Self {
        Material::new()
    }
    fn draw(&self, renderer: &Renderer) {
        self.ornament.draw(renderer);
        self.knight.draw(renderer);
        self.parwns.iter().for_each(|parwn| {
           parwn.draw(renderer);
        });
        self.shots.iter().for_each(|shot| {
            shot.draw(renderer);
        });
        self.fires.iter().for_each(|fire| {
            fire.draw(renderer);
        });
        self.bishops.iter().for_each(|bishop| {
            bishop.draw(renderer);
        });
        self.rooks.iter().for_each(|rook| {
            rook.draw(renderer);
        });
    }
}

#[async_trait(?Send)]
impl Game for GameStage {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        log!("START");
        match &self.machine {
            _none => {
                let machine = GameStageStateMachine::new(Material::new());
                Ok(Box::new(GameStage {
                    machine: Some(machine),
                }))
            }
            Some(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    // Whole World UPDATE
    fn update(&mut self, _keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(_keystate));
        }
        assert!(self.machine.is_some());
    }
    // Whole World Drawing
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0,
            y: 0,
            width: 600,
            height: 600,
            character: BRANK,
            font_size: FONT_L,
            font_align: FONT_CENTER,
        });
        match &self.machine {
            Some(GameStageStateMachine::Ready(_state)) => {
                if _state.material.frame < MESSAGE_TIME {
                    renderer.draw_text(
                        &Rect {
                            x: MESSAGE_X,
                            y: MESSAGE_Y,
                            width: MESSAGE_WIDTH,
                            height: MESSAGE_HEIGHT,
                            character: MESSAGE_OPENING,
                            font_size: FONT_S,
                            font_align: FONT_LEFT,
                        },
                    );
                }
            }
            Some(GameStageStateMachine::Playing(_state)) => {
                if _state.material.frame < MESSAGE_TIME {
                    renderer.draw_text(
                        &Rect {
                            x: MESSAGE_X,
                            y: MESSAGE_Y,
                            width: MESSAGE_WIDTH,
                            height: MESSAGE_HEIGHT,
                            character: MESSAGE_RUNNING,
                            font_size: FONT_S,
                            font_align: FONT_LEFT,
                        },
                    );
                }
            }
            _=> {
                renderer.draw_text(
                    &Rect {
                        x: MESSAGE_X,
                        y: MESSAGE_Y,
                        width: MESSAGE_WIDTH,
                        height: MESSAGE_HEIGHT,
                        character: MESSAGE_GAMEOVER,
                        font_size: FONT_S,
                        font_align: FONT_LEFT,
                    },
                );
            }
        }
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}