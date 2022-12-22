pub mod map_generator;

fn main() {
    let map_height = 30;
    let map_width = 30;

    let result: Vec<Vec<char>> = map_generator::render_map(
        &map_generator::generate(map_width, map_height, 0.8, 98),
        map_width,
        map_height,
    );

    for row in &result {
        for element in row {
            print!("{}", element);
        }
        println!();
    }
}
