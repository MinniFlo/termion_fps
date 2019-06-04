use super::GameState;
use super::structs::{Player};

pub fn step_calculation(step: f32, angel: f32) -> (f32, f32) {
    let x_pos = angel.to_radians().cos() * step;
    let y_pos = angel.to_radians().sin() * step;
    (x_pos, y_pos)
}

pub fn calc_render_map(game: &mut GameState) -> String {
    let start_angel : f32 = (game.player.angel + game.player.fov as f32 / 2.0) % 360.0; // start of the players fov
    let step_size : f32 = game.player.fov as f32 / game.render_win.size.0 as f32; // amount by witch the start_angel is increased
    let mut final_str = String::new();


    for i in 0..game.render_win.size.0 { // iterates over each part of the fov
        let cur_angel : f32 = start_angel + 360.0 - i as f32 * step_size % 360.0;
        let mut distance : f32 = 0.0; // travled distance
        let (mut dist_x, mut dist_y) = game.player.koordinates; // coordinates of distant point
        let mut hit_char : char = ' '; // char of point (dist_x, dist_y)
        let (step_x, step_y) = step_calculation(0.1, cur_angel); // unit vector for the ray-trace steps
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

        // try to smooth out the curves does not work yet
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
            let mut wall_height = calc_wall_height(distance, game.render_win.size.1, game.render_dist, 0.25);

            // calculate the number of cells that is not wall
            let screen_remainder = game.render_win.size.1 - wall_height as u16;
            let floor_sky_size = (screen_remainder / 2) as usize;
            let size_texture_diff: i16 = floor_sky_size as i16 - game.floor_texture.len() as i16;
            let mut floor_vec = Vec::new();

            if size_texture_diff > 0 {
                floor_vec.append(&mut vec![' '; size_texture_diff as usize]);
                floor_vec.append(&mut game.floor_texture);
            } else if size_texture_diff < 0 {
                let texture_index = (size_texture_diff.abs() - 1) as usize;
                floor_vec.append(&mut game.floor_texture[texture_index ..].to_vec());
            } else {
                floor_vec.append(&mut game.floor_texture);
            }

            let mut the_vec = Vec::new();

            // puts vec's together
            if screen_remainder % 2 == 0 {
                the_vec = vec![' '; floor_sky_size];
                the_vec.append(&mut vec![display_char; (wall_height) as usize]);
                the_vec.append( &mut floor_vec);
            } else {
                let floor_num = (screen_remainder / 2) as usize;
                the_vec = vec![' '; floor_num + 1];
                the_vec.append(&mut vec![display_char; (wall_height) as usize]);
                the_vec.append(&mut floor_vec);
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
pub fn calc_wall_height(distance: f32, win_size: u16, render_dist: u32, min_height: f32) -> u32 {
    (-((win_size as f32 - win_size as f32 * min_height) / render_dist as f32) * distance + win_size as f32) as u32
}

pub fn flatter(distance: f32, player: &Player, cur_angel: f32) -> f32 {
    ((cur_angel - player.angel as f32 + 90.0).to_radians().sin() * distance).abs()
}