use std::collections::HashMap;

use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::Rng;

pub fn generate(
    width: usize,
    height: usize,
    rock_threshold: f64,
    small_rock_spawn_rate: u16,
) -> HashMap<(usize, usize), ObstacleType> {
    // Init rng
    let mut rng = rand::thread_rng();

    // Init fbm
    let fbm = Fbm::<Perlin>::new(rng.gen()).set_octaves(2);

    let mut map: HashMap<(usize, usize), ObstacleType> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            // Get noise value
            let val = fbm
                .get([x as f64 / width as f64, y as f64 / height as f64])
                .abs();

            let obstacle_type: ObstacleType = if val > rock_threshold {
                ObstacleType::Rock
            } else {
                let small_rock_odd = rng.gen_range(0..100);
                if small_rock_odd > small_rock_spawn_rate {
                    ObstacleType::SmallRock
                } else {
                    ObstacleType::None
                }
            };

            // Add to hashmap
            map.insert((x, y), obstacle_type);
        }
    }

    return map;
}

pub fn render_map(
    map: &HashMap<(usize, usize), ObstacleType>,
    width: usize,
    height: usize,
) -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = vec![vec![' '; width]; height];

    for x in 0..width {
        for y in 0..height {
            match map.get(&(x, y)) {
                Some(ObstacleType::Rock) => {
                    // TODO: Fix flipped x and y
                    // Check neighbors
                    let mut up_empty = false;
                    if x > 0 {
                        up_empty = check_empty(map, (x - 1, y));
                    }

                    let mut down_empty = false;
                    if x < width {
                        down_empty = check_empty(map, (x + 1, y));
                    }

                    let mut left_empty = false;
                    if y < width {
                        left_empty = check_empty(map, (x, y + 1));
                    }

                    let mut right_empty = false;
                    if y > 0 {
                        right_empty = check_empty(map, (x, y - 1));
                    }

                    // TODO: optimize logic
                    if right_empty && up_empty {
                        array[x][y - 1] = '/';
                    } else if left_empty && up_empty {
                        array[x][y + 1] = '\\';
                    } else if right_empty && down_empty {
                        array[x][y - 1] = '\\';
                    } else if left_empty && down_empty {
                        array[x][y + 1] = '/';
                    }

                    if left_empty {
                        if array[x][y + 1] == ' ' {
                            array[x][y + 1] = '|';
                        }
                    }
                    if right_empty {
                        if array[x][y - 1] == ' ' {
                            array[x][y - 1] = '|';
                        }
                    }
                    if up_empty {
                        if array[x - 1][y] == ' ' {
                            array[x - 1][y] = '_';
                        }
                    }
                    if down_empty {
                        if array[x][y] == ' ' {
                            array[x][y] = '_';
                        }
                    }

                    // if array[x][y] == String::from(" ") {
                    //     array[x][y] = String::from("@");
                    // }
                }
                Some(ObstacleType::SmallRock) => array[x][y] = '*',
                Some(ObstacleType::None) => (),
                _ => (),
            }
        }
    }

    let mut player_spawned = false;
    let mut goal_spawned = false;

    let mut rng = rand::thread_rng();

    while !player_spawned {
        let random_x = rng.gen_range(0..width);
        let random_y = rng.gen_range(0..height);

        if let Some(ObstacleType::None) = map.get(&(random_x, random_y)) {
            array[random_x][random_y] = 'R';
            player_spawned = true;
        }
    }

    while !goal_spawned {
        let random_x = rng.gen_range(0..width);
        let random_y = rng.gen_range(0..height);

        if let Some(ObstacleType::None) = map.get(&(random_x, random_y)) {
            if array[random_x][random_y] == ' ' {
                array[random_x][random_y] = 'X';
                goal_spawned = true;
            }
        }
    }

    return array;
}

fn check_empty(map: &HashMap<(usize, usize), ObstacleType>, point: (usize, usize)) -> bool {
    if map.contains_key(&point) {
        match map.get(&point) {
            Some(ObstacleType::None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    return false;
}

pub enum ObstacleType {
    None,
    Rock,
    SmallRock,
}
