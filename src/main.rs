extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, Read, stdout};
use termion::{cursor, clear, async_stdin};
use std::time::{Instant, Duration};
use std::thread::sleep;

mod game;
use game::GameState;
use game::{structs, renderLogic};


fn main() {

    // logic setup
    let mut player = game::structs::Player{koordinates: (2.0, 13.5), angel: 0.0, fov: 60};
    let map_win = structs::Window{size: (15, 15), start: (0, 7)};
    let render_win = structs::Window{size: (350, 120), start: (33, 3)};
                                    //   0    1    2    3    4    5    6    7    8    9    0    1    2    3    4
    let map_vec = vec!(vec!('#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'));

    let mut render_vec: Vec<Vec<char>> = vec!(vec!('*'; render_win.size.1 as usize); render_win.size.0 as usize);

    let mut floor_texture: Vec<char> = Vec::new();

    let floor_height = (render_win.size.1 as f32 * 0.2) as u16;

    for i in (0 .. (floor_height as f32 * 0.3) as u16).rev() {
        if i as f32 <= floor_height as f32 * 0.3 {
            floor_texture.push('.');
        } else if i as f32 <= floor_height as f32 * 0.2 {
            floor_texture.push('+');
        } else if i as f32 <= floor_height as f32 * 0.1 {
            floor_texture.push('#');
        }
    }

    let mut game = GameState{player, 
                            map_win, 
                            render_win,
                            map_vec, 
                            render_vec,
                            floor_texture,
                            render_dist: 12 };

    // termion setup
    let mut stdin = async_stdin().bytes();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(stdout, "{}{}",
           clear::All,
           cursor::Hide
    ).unwrap();

    // first instant to set frame rate
    let mut start_stemp = Instant::now();

    // flag that prevents pointless rendering
    let mut render_pls = true;

    let mut the_str = String::new();

    // main loop
    loop {
        // puts next char from input buffer in to c
        let c = stdin.next();

        // binds player coordinates to vars for better readability
        let (x, y) = game.player.koordinates;


        // evaluates input
        match c {
            Some(Ok(b'q')) => break,
            Some(Ok(b'w')) => {
                let (step_x, step_y) = renderLogic::step_calculation(0.25, game.player.angel);
                if game.map_vec[(y - step_y) as usize][(x + step_x) as usize] != '#' {
                    game.player.koordinates = (x + step_x, y - step_y);
                    render_pls = true;
                }
            },
            Some(Ok(b's')) => {
                let (step_x, step_y) = renderLogic::step_calculation(-0.25, game.player.angel);
                if game.map_vec[(y - step_y) as usize][(x + step_x) as usize] != '#' {
                    game.player.koordinates = (x + step_x, y - step_y);
                    render_pls = true;
                }
            },
            Some(Ok(b'a')) => {
                game.player.angel = (game.player.angel + 2.0) % 360.0;
                render_pls = true;
            },
            Some(Ok(b'd')) => {
                game.player.angel = (game.player.angel + 360.0 - 2.0) % 360.0;
                render_pls = true;
            },
            _ => {},
        }

        // creates new Image
        if render_pls {
            the_str = renderLogic::calc_render_map(&mut game);
        }

        // takes current Instant to set frame rate
        let now = Instant::now();

        // sets frame rate to 1 pic per 17 ms
        if now.duration_since(start_stemp).as_millis() >= 17 && render_pls{

            // set up
            write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

            // player informations
            write!(stdout, "x: {} | y: {} | angel: {}\n",
                   game.player.koordinates.0,
                   game.player.koordinates.1,
                   game.player.angel).unwrap();

            // map
            for field_y in 0..game.map_win.size.1 {
                for field_x in 0..game.map_win.size.0 {
                    write!(stdout, "{}", cursor::Goto((field_x + game.map_win.start.0) * 2, field_y + game.map_win.start.1)).unwrap();
                    if field_x == x as u16 && field_y == y as u16 {
                        write!(stdout, "{}\n", '*').unwrap();
                        continue
                    }
                    write!(stdout, "{}\n", game.map_vec[field_y as usize][field_x as usize]).unwrap();
                }
            }

            // view
            write!(stdout, "{}", the_str).unwrap();

            stdout.flush().unwrap();

            // resets time stemp
            start_stemp = Instant::now();
            render_pls = false;
        } else if now.duration_since(start_stemp).as_millis() < 17 && render_pls{
            sleep(Duration::from_millis((17 - now.duration_since(start_stemp).as_millis()) as u64))
        } else {
            sleep( Duration::from_millis(5))
        }
    }
    write!(stdout, "{}", cursor::Show).unwrap();
}
