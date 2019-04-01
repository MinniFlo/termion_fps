extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, Read, stdout};
use termion::{cursor, clear, async_stdin};
use std::thread;
use std::time::{Duration, Instant};

struct GameState {
    player: Player,
    map_win: Window,
    render_win: Window,
    map_vec: Vec<Vec<char>>,
    render_vec: Vec<Vec<char>>,
    render_dist: u32
    }

struct Player {koordinates: (f32, f32), angel: f32, fov: u16}

struct Window {size: (u16, u16), start: (u16, u16)}

fn step_calculation(step: f32, angel: f32) -> (f32, f32) {
    let x_pos = angel.to_radians().cos() * step;
    let y_pos = angel.to_radians().sin() * step;
    (x_pos, y_pos)
}

fn calc_render_map(game: &mut GameState) -> String {
    let start_angel : f32 = (game.player.angel + game.player.fov as f32 / 2.0) % 360.0; // start of the players fov
    let step_size : f32 = game.player.fov as f32 / game.render_win.size.0 as f32; // amount by witch the start_angel is increased
    let mut final_str = String::new();


    for i in 0..game.render_win.size.0 { // iterates over each part of the fov
        let cur_angel : f32 = start_angel + 360.0 - i as f32 * step_size % 360.0;
        let mut distance : f32 = 0.0; // travled distance
        let (mut dist_x, mut dist_y) = game.player.koordinates; // coordinates of distant point
        let mut hit_char : char = ' '; // char of point (dist_x, dist_y)
        let (step_x, step_y) = step_calculation(0.1, (cur_angel)); // unit vector for the ray-trace steps
        let mut max_dist_reached = false; // flag for the next loop
        let mut display_char = ' ';

        // ray-trace
        while hit_char != '#' {
            dist_x += step_x;
            dist_y -= step_y; // set new coordinates
            distance += 0.1; // increase distance

            if distance >= game.render_dist as f32 || dist_y >= 15.0 || dist_x >= 15.0 || dist_y <= 0.0 || dist_x <= 0.0 { // checks if max distance is reached
                max_dist_reached = true;
                break;
            }

            hit_char = game.map_vec[dist_y as usize][dist_x as usize]; // sets new char

        }

//         distance = flatter(distance, &game.player, cur_angel);

        // build vertical Vectors
        if max_dist_reached {
            game.render_vec[i as usize] = vec!(' '; game.render_win.size.1 as usize);
        } else {
            // changes display-char for walls in relation to the distance
            match () {
                _ if distance <= game.render_dist as f32 * 0.15 => display_char = '\u{2588}',
                _ if distance <= game.render_dist as f32 * 0.35 => display_char = '\u{2593}',
                _ if distance <= game.render_dist as f32 * 0.6 => display_char = '\u{2592}',
                _ => display_char = '\u{2591}'
            }

            // calculate wall height
            let mut wall_height = calc_wall_height(distance, game.render_win.size.1, game.render_dist, 0.15);

            // calculate the number of cells that is not wall
            let screen_remainder = game.render_win.size.1 - wall_height as u16;
            let floor_size = screen_remainder / 2;
            let mut the_vec = Vec::new();

            // puts vec's together
            if screen_remainder % 2 == 0 {
                let mut sky_floor_vec = vec!(' '; (screen_remainder / 2) as usize);
                the_vec = sky_floor_vec.clone();
                the_vec.append(&mut vec!(display_char; (wall_height) as usize));
                the_vec.append( &mut sky_floor_vec);
            } else {
                let floor_num = (screen_remainder / 2) as usize;
                the_vec = vec!(' '; floor_num + 1);
                the_vec.append(&mut vec!(display_char; (wall_height) as usize));
                the_vec.append(&mut vec!(' '; floor_num));
            }
            
            for chr in the_vec {
                final_str.push(chr);
                final_str.push_str("\n\x1B[1D");
            }
            final_str.push_str("\x1B[1C\x1B[120A");
        }
    }
    return final_str;
}

// linear function witch calculates the wall-size in relation to the distance
fn calc_wall_height(distance: f32, win_size: u16, render_dist: u32, min_height: f32) -> u32 {
    (-((win_size as f32 - win_size as f32 * min_height) / render_dist as f32) * distance + win_size as f32) as u32
}

fn flatter(distance: f32, player: &Player, cur_angel: f32) -> f32 {
    ((cur_angel - player.angel as f32 + 90.0).to_radians().sin() * distance).abs()
}

fn main() {

    // logic setup
    let mut player = Player{koordinates: (1.0, 13.5), angel: 350.0, fov: 50};
    let map_win = Window{size: (15, 15), start: (0, 7)};
    let render_win = Window{size: (350, 120), start: (33, 3)};
                                    //   0    1    2    3    4    5    6    7    8    9    0    1    2    3    4
    let map_vec = vec!(vec!('#', ' ', ' ', ' ', ' ', '#', ' ', ' ', '#', '#', '#', '#', '#', '#', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' '),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'),
                                    vec!('#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'));

    let mut render_vec: Vec<Vec<char>> = vec!(vec!('*'; render_win.size.1 as usize); render_win.size.0 as usize);

    let floor_texture: Vec<char> = vec![];

    let mut game = GameState{player, 
                            map_win, 
                            render_win,
                            map_vec, 
                            render_vec,
                            render_dist: 15 };

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
                let (step_x, step_y) = step_calculation(0.25, game.player.angel);
                if game.map_vec[(y - step_y) as usize][(x + step_x) as usize] != '#' {
                    game.player.koordinates = (x + step_x, y - step_y);
                    render_pls = true;
                }
            },
            Some(Ok(b's')) => {
                let (step_x, step_y) = step_calculation(-0.25, game.player.angel);
                if game.map_vec[(y - step_y) as usize][(x + step_x) as usize] != '#' {
                    game.player.koordinates = (x + step_x, y - step_y);
                    render_pls = true;
                }
            },
            Some(Ok(b'a')) => {game.player.angel = (game.player.angel + 2.0) % 360.0;
                                render_pls = true;},
            Some(Ok(b'd')) => {game.player.angel = (game.player.angel + 360.0 - 2.0) % 360.0;
                                render_pls = true;},
            _ => {},
        }

        // creates new Image
        let the_str = calc_render_map(&mut game);

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
        }
    }
    write!(stdout, "{}", cursor::Show).unwrap();
}
