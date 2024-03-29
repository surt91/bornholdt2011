extern crate piston;
extern crate glutin_window;
// extern crate sdl2_window;
extern crate opengl_graphics;
extern crate fps_counter;
extern crate rand;

use super::graphics::*;
// use self::sdl2_window::Sdl2Window as Window;
use self::glutin_window::GlutinWindow as Window;
use self::piston::window::WindowSettings;
use self::piston::input::keyboard::Key::*;

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::event_loop::{Events, EventSettings};
use self::piston::input::Event::{Loop, Input};
use self::piston::input::Loop::{Render, Update};
use self::piston::input::Input::Button;
use self::piston::input::Button::Keyboard;
use self::piston::input::ButtonState::Press;
use self::fps_counter::FPSCounter;

use super::Model;

/// convert a hsv color representations into a rgb representation
fn hsv2rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f64;
    let p = v*(1.-s);
    let q = v*(1.-s*f);
    let t = v*(1.-s*(1.-f));

    match hi {
        0 | 6 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0., 0., 0.)
    }
}

pub fn opinion_to_color(opinion: usize) -> types::Color {
    let (r, g, b) = hsv2rgb(((opinion*30) % 187) as f64 / 187., 1., 1.);
    [r as f32, g as f32, b as f32, 1.]
}

pub trait Renderable {
    fn render<G>(&self, c: &Context, gfx: &mut G, size: &(u32, u32))
        where G: Graphics;
}

pub fn show<T: Renderable + Model>(model: &mut T) {
    let size = (model.l() as u32 * 5, model.l() as u32 * 5);
    let mut window: Window = WindowSettings::new("Bornholdt", [size.0, size.1])
                                            .exit_on_esc(true)
                                            .decorated(false)
                                            .srgb(false)
                                            .build()
                                            .unwrap();

    let mut gfx = GlGraphics::new(OpenGL::V3_2);
    let mut fps = FPSCounter::new();
    let mut sweeps_per_second = 100.;
    let mut rate = 0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Loop(x) => match x {
                Render(args) => {
                    rate = fps.tick();

                    gfx.draw(args.viewport(), |c, gfx| {
                        model.render(&c, gfx, &size);
                    });
                },

                Update(args) => {
                    model.sweep((args.dt * sweeps_per_second).ceil() as usize);
                },

                _ => ()
            },

            Input(x) => match x {
                Button(b) =>
                {
                    if b.state == Press {
                        match b.button {
                            Keyboard(key) => match key {
                                F => println!("{} FPS", rate),
                                S => println!("{} Sweeps", model.total_sweeps()),
                                P => if sweeps_per_second == 0. {sweeps_per_second = 100.} else {sweeps_per_second = 0.},
                                Up => {
                                    sweeps_per_second *= 1.2;
                                    println!("{:.0} sweeps per second", sweeps_per_second);
                                }
                                Down => {
                                    sweeps_per_second /= 1.2;
                                    println!("{:.0} sweeps per second", sweeps_per_second);
                                }
                                _ => ()
                            },
                            _ => ()
                        };
                    }
                },
                _ => ()
            },
            _ => {}
        }
    }
}
