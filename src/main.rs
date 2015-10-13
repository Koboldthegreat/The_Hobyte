extern crate piston;
use piston::input::*;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate image as ImageLib;

extern crate find_folder;

use piston::window::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL, Texture};
use graphics::*;
use ImageLib::{GenericImage, ImageBuffer};

use std::fs::File;
use std::path::Path;


pub struct Cube {
    rotation: f64,
    x: f64,
    y: f64,
    old_pos: (f64, f64),
    velocity_y: f64,
    velocity_x: f64,
    bounding_rect: types::Rectangle,
}

pub struct Wall {
    rotation: f64,
    x: f64,
    y: f64,
}

pub struct App {
    gl: GlGraphics,
    player: Cube,
    enemy: Cube,
    actions: Actions,
    wall: Wall,


}

#[derive(Default)]
struct Actions {
    left: bool,
    right: bool,
    jump: bool,
    sneak: bool

}

impl App {
    fn collides(&mut self) -> bool{
        if self.player.x > self.wall.x + 48.0 {return false; };
        if self.player.x < self.wall.x - 48.0 {return false; };
        if self.player.y > self.wall.y + 50.0 {return false; };
        if self.player.y < self.wall.y - 50.0 {return false; };
        return true
    }

    fn collides_side(&mut self) -> bool {
        if self.player.x > self.wall.x + 48.0 {return false; };
        if self.player.x < self.wall.x - 48.0 {return false; };
        return true
    }
    fn collides_top(&mut self) -> bool{
        if self.player.y > self.wall.y + 50.0 {return false; };
        if self.player.y < self.wall.y - 50.0 {return false; };
        return true
    }
    fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Left => self.actions.left = pressed,
            Key::Right => self.actions.right = pressed,
            Key::Up => self.actions.jump = pressed,
                _ => ()
            }
        }

    fn render(&mut self, args: &RenderArgs){


        const WHITE: [f32; 4] = [240.0/255.0, 240.0/255.0, 240.0/255.0, 1.0];
        const ORANGE: [f32; 4] = [244.0/255.0, 81.0/255.0, 30.0/255.0, 1.0];
        const GREEN: [f32; 4] = [139.0/255.0, 195.0/255.0, 74.0/255.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.player.rotation;
        let (x, y) = (self.player.x, self.player.y);

        let (wallx, wally) = (self.wall.x, self.wall.y);

        let enemyrotation = self.enemy.rotation;
        let (enemyx, enemyy) = (self.enemy.x, self.enemy.y);

        self.gl.draw(args.viewport(), |c, gl| {
            //clear the screen
            clear(WHITE, gl);

            let ptransform = c.transform.trans(x, y)
                                        .rot_rad(rotation)
                                        .trans(-25.0, -25.0);
            // Draw a box rotating around the middle of the screen

            rectangle(ORANGE, square, ptransform, gl)
        });

        self.gl.draw(args.viewport(), |c, gl| {

            let etransform = c.transform.trans(enemyx, enemyy)
                                        .rot_rad(enemyrotation)
                                        .trans(-25.0, -25.0);
            // Draw a box rotating around the middle of the screen

            rectangle(GREEN, square, etransform, gl)
        });

        self.gl.draw(args.viewport(), |c, gl| {

            let etransform = c.transform.trans(wallx, wally)
                                        .rot_rad(enemyrotation)
                                        .trans(-25.0, -25.0);
            // Draw a box rotating around the middle of the screen

            rectangle(GREEN, square, etransform, gl)
        });
    }
    fn update(&mut self, args: &UpdateArgs){

        if self.actions.left{
            self.player.velocity_x = -5.0;
            } else if self.player.velocity_x < -1.5 {
                self.player.velocity_x = -1.5;
            }
        if self.actions.right{
                self.player.velocity_x = 5.0;
                } else if self.player.velocity_x > 1.5 {
                    self.player.velocity_x = 1.5;
                }
        if self.actions.jump{
                    self.player.velocity_y = -5.0;
                }

        if self.player.velocity_y < 9.81 * 35.0 {
            self.player.velocity_y += 15.0 * args.dt;
        }

        if self.player.velocity_x > 0.0{
            self.player.velocity_x -= 7.0 * args.dt;
            } else if self.player.velocity_x > -1.0 && self.player.velocity_x < 1.0{
                self.player.velocity_x = 0.0;
            } else {
                self.player.velocity_x += 7.0 * args.dt;
            }

        //apply velocity
        self.player.y += self.player.velocity_y;
        self.player.x += self.player.velocity_x;

        if self.player.y > 1000.0 -25.0 {
            self.player.y = 1000.0 - 25.0 ;
        }


        if self.collides(){
            self.player.velocity_x = 0.0;
            self.player.velocity_y = 0.0;
            let mut y = self.player.y/50.0;
            let mut x = self.player.x/50.0;
            y = y.round();
            x = x.round();


            if self.collides_top(){
                self.player.y = y*50.0 - 25.0;
            }

            if self.player.x < self.wall.x && self.collides_side(){
                self.player.x = x*50.0 - 25.0;
            }

            if self.player.x > self.wall.x && self.collides_side(){
                self.player.x = x*50.0 + 25.0;
            }
        }

        //
        //println!("collide: {}", self.collides());
        //println!("velocityY: {}", self.player.velocity_y);
        //

        }
    }



fn main(){




    // Change this to OpenGL::v2_1 if not working
    let opengl= OpenGL::V3_2;

    // Create an Glutin Window
    let window: GlutinWindow = WindowSettings::new(
            "spinning-square",
            [1000, 1000]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let font = &assets.join("FiraSans-Regular.ttf");

    let level = assets.join("level.png");

    let level = ImageLib::open(Path::new(&level)).unwrap();

    print!("{:?}", level.get_pixel(1,1));
    let mut ny = 0.0;

    let mut wall = Wall {
            rotation: 0.0,
            x: 400.0,
            y: 1000.0 - 25.0
    };

    let mut enemy = Cube {
            rotation: 0.0,
            x: 100.0,
            y: 100.0,
            old_pos: (100.0, 100.0),
            velocity_y: 0.0,
            velocity_x: 0.0,
            bounding_rect: rectangle::square(100.0, 100.0, 50.0)
    };

    for y in 0..level.height(){
        let mut nx = 0.0;
        for x in 0..level.width(){
            if level.get_pixel(x, y).data == [0, 0, 0, 255]{
                wall.x = 50.0 * nx + 25.0;
                wall.y = 50.0 * ny + 25.0;
                println!("yas");
            }
            else if level.get_pixel(x, y).data == [255, 0, 0, 255]{
                enemy.x = 50.0 * nx + 25.0;
                enemy.y = 50.0 * ny + 25.0;

            }
            //println!("{}x,{}y: {:?}", x, y, level.get_pixel(x, y));
            nx = nx + 1.0;
        }
        ny = ny + 1.0;
    }



    let mut player = Cube {
            rotation: 0.0,
            x: 100.0,
            y: 100.0,
            old_pos: (100.0, 100.0),
            velocity_y: 0.0,
            velocity_x: 0.0,
            bounding_rect: rectangle::square(100.0, 100.0, 50.0)
    };





    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: player,
        enemy: enemy,
        actions: Actions::default(),
        wall: wall
    };




    for e in window.events(){


        if let Some(u) = e.update_args(){
            app.update(&u);
        };



        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                    app.key_press(key);
                }
            Event::Input(Input::Release(Button::Keyboard(key))) => {
                    app.key_release(key);
            }


            Event::Render(args) => {
                app.render(&args);
            }

            _ => {}
        }

       }
}
