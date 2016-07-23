//! Example of components.

extern crate amethyst;

use amethyst::engine::{Application, State, Trans};
use amethyst::context::{Context, Config};
use amethyst::config::Element;
use amethyst::ecs::{Join, World};
use amethyst::components::Position1;
use amethyst::context::event::{EngineEvent, Event, VirtualKeyCode};
use amethyst::ecs::{JoinIter, Entity};
struct Example;

impl State for Example {
    fn handle_events(&mut self, events: Vec<Entity>, context: &mut Context, world: &mut World) -> Trans {
        let mut trans = Trans::None;
        let storage = context.broadcaster.read::<EngineEvent>();
        for _event in events {
            let event = storage.get(_event).unwrap();
            let event = &event.payload;
            match *event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::A)) => {
                    let mut storage = world.write::<Position1>();
                    for p in JoinIter::new((&mut storage)) {
                        p.0 += 10.;
                    }
                },
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => trans = Trans::Quit,
                Event::Closed => trans = Trans::Quit,
                _ => (),
            }
        }
        trans
    }
    fn on_start(&mut self, context: &mut Context, world: &mut World) {
        use amethyst::context::video_context::VideoContext;
        use amethyst::renderer::pass::*;
        use amethyst::renderer::Layer;
        println!("Initializing");
        world.register::<Position1>();
        world.create_now().with(Position1::new(10.)); 
        world.create_now().with(Position1::new(5.));
        match context.video_context {
            VideoContext::OpenGL { ref mut frame, .. } => {
                let clear_layer =
                    Layer::new("main",
                               vec![
                                   Clear::new([0., 0., 0., 1.]),
                               ]);
                frame.layers.push(clear_layer);
            }
            #[cfg(windows)]
            VideoContext::Direct3D {  } => {
                // stub
            },
            VideoContext::Null => (),
        }
    }

    fn update(&mut self, context: &mut Context, _: &mut World) -> Trans {
        use amethyst::context::video_context::VideoContext;
        match context.video_context {
            VideoContext::OpenGL { ref window,
                                   ref mut renderer,
                                   ref frame,
                                   ref mut device,
                                   .. } => {
                renderer.submit(frame, device);
                window.swap_buffers().unwrap();
            }
#[cfg(windows)]
            VideoContext::Direct3D {  } => {
                // stub
            },
            VideoContext::Null => (),
        }
        Trans::None
    }

    fn on_stop(&mut self, _: &mut Context, world: &mut World) {
        let pos = world.read::<Position1>();
        let entities: Vec<Entity> = world.entities().iter().map(|e| e.clone()).collect();
        
        println!("End! The entities are now at");
        for entity in entities {
            println!("{:?}", pos.get(entity).unwrap().0);
        }
    }
}

fn main() {
    // Since there is no support for TTY events yet, we have to use following
    // instead of: 
    // let config = Config::default();
    let config = Config::from_file("../config/window_example_config.yml").unwrap();
    let mut game = Application::build(Example, config).done();
    game.run();
}
