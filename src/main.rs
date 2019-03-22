extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, Read, stdout};
use termion::{cursor, clear, async_stdin};
use std::thread;
use std::time::Duration;

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

fn calc_render_map(game: &mut GameState) {
    let start_angel : f32 = (game.player.angel + (360 - game.player.fov / 2) as f32) % 360.0; // start of the players fov
    let step_size : f32 = game.player.fov as f32 / game.render_win.size.0 as f32; // amount by witch the start_angel is increased

    for i in 0..game.render_win.size.0 { // iterates over each part of the fov
        let mut distance : f32 = 0.0; // travled distance
        let (mut dist_x, mut dist_y) = game.player.koordinates; // coordinates of distant point
        let mut hit_char : char = ' '; // char of point (dist_x, dist_y)
        let (step_x, step_y) = step_calculation(0.1, (start_angel + i as f32 * step_size % 360.0)); // unit vector for the ray-trace steps
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

            // puts vec's together
            if screen_remainder % 2 == 0 {
                let mut sky_floor_vec = vec!(' '; (screen_remainder / 2) as usize);
                game.render_vec[i as usize] = sky_floor_vec.clone();
                game.render_vec[i as usize].append(&mut vec!(display_char; (wall_height) as usize));
                game.render_vec[i as usize].append( &mut sky_floor_vec);
            } else {
                let floor_num = (screen_remainder / 2) as usize;
                game.render_vec[i as usize] = vec!(' '; floor_num + 1);
                game.render_vec[i as usize].append(&mut vec!(display_char; (wall_height) as usize));
                game.render_vec[i as usize].append(&mut vec!(' '; floor_num));
            }
        }
    }

    // is needed because most left vector's are build at first
    game.render_vec.reverse()
}

// linear function witch calculates the wall-size in relation to the distance
fn calc_wall_height(distance: f32, win_size: u16, render_dist: u32, min_height: f32) -> u32 {
    (-((win_size as f32 - win_size as f32 * min_height) / render_dist as f32) * distance + win_size as f32) as u32
}

fn main() {

    // logic setup
    let mut player = Player{koordinates: (1.0, 13.5), angel: 350.0, fov: 50};
    let map_win = Window{size: (15, 15), start: (0, 3)};
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
                            render_dist: 10 };




    // termion setup
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}",
           clear::All,
           cursor::Hide
    ).unwrap();

    // prints player-information
    write!(stdout, "{}", cursor::Goto(1,1)).unwrap();
    write!(stdout, "x: {} | y: {} | angel: {}\n",
           game.player.koordinates.0,
           game.player.koordinates.1,
           game.player.angel).unwrap();

    // prints map
    for field_y in 0..game.map_win.size.1 {
        for field_x in 0..game.map_win.size.0 {
            write!(stdout ,"{}", cursor::Goto((field_x + game.map_win.start.0) * 2, field_y + game.map_win.start.1)).unwrap();
            if field_x == game.player.koordinates.0 as u16 && field_y == game.player.koordinates.1 as u16 {
                write!(stdout, "{}\n", '*').unwrap();
                continue
            }
            write!(stdout, "{}\n", game.map_vec[field_y as usize][field_x as usize]).unwrap();
        }
    }
    // builds the view
    calc_render_map(&mut game);

    // prints view
    for field_x in (0..game.render_win.size.0) {
        for field_y in 0..game.render_win.size.1 {
            write!(stdout ,"{}", cursor::Goto(field_x + game.render_win.start.0, field_y + game.render_win.start.1)).unwrap();
            write!(stdout, "{}", game.render_vec[field_x as usize][field_y as usize]).unwrap();
        }
    }

    // update terminal
    stdout.flush().unwrap();

//    let mut timer = 100;
//
//    loop {
//        let c = stdin.next();
//
//        let (x, y) = game.player.koordinates;
//
//        match c {
//            Some(Ok(b'q')) => break,
//            Some(Ok(b'w')) => {let (step_x, step_y) = step_calculation(0.25, game.player.angel);
//                                if game.map_vec[(y - step_y) as usize][(x + step_x) as usize] != '#' {
//                                    game.player.koordinates = (x + step_x, y - step_y);
//                                }},
//            Some(Ok(b's')) => {let (step_x, step_y) = step_calculation(-0.25, game.player.angel);
//                                if game.map_vec[(y - step_y) as usize][(x + step_x) as usize] != '#' {
//                                    game.player.koordinates = (x + step_x, y - step_y);
//                                }},
//            Some(Ok(b'a')) => game.player.angel = (game.player.angel + 10.0) % 360.0,
//            Some(Ok(b'd')) => game.player.angel = (game.player.angel + 360.0 -10.0) % 360.0,
//            None => {},
//            _ => {},
//        }
//
//        if timer == 50 {
//            calc_render_map(&mut game);
//
//            for field_y in 0..game.render_win.size.0 {
//                for field_x in 0..game.render_win.size.1 {
//                    write!(stdout ,"{}", cursor::Goto((field_x + game.render_win.start.0), field_y + game.render_win.start.1)).unwrap();
//                    write!(stdout, "{}\n", game.render_vec[field_y as usize][field_x as usize]).unwrap();
//                }
//            }
//
//            stdout.flush().unwrap();
//        }
//
//        if timer == 100 {
//            write!(stdout, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
//            write!(stdout, "x: {} | y: {} | angel: {}\n",
//                   game.player.koordinates.0,
//                   game.player.koordinates.1,
//                   game.player.angel).unwrap();
//
//            for field_y in 0..game.map_win.size.1 {
//                for field_x in 0..game.map_win.size.0 {
//                    write!(stdout ,"{}", cursor::Goto((field_x + game.map_win.start.0) * 2, field_y + game.map_win.start.1)).unwrap();
//                    if field_x == x as u16 && field_y == y as u16 {
//                        write!(stdout, "{}\n", '*').unwrap();
//                        continue
//                    }
//                    write!(stdout, "{}\n", game.map_vec[field_y as usize][field_x as usize]).unwrap();
//                }
//            }
//
//            stdout.flush().unwrap();
//            timer = 0;
//        }
//        timer += 1;
//
//        thread::sleep(Duration::from_millis(1));
//    }
    write!(stdout, "{}", cursor::Show).unwrap();
}