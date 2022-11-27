

fn main() {
    show_most_level_colors();
}

fn show_most_level_colors() {
    const COLORS: [(u8, u8, u8); 5] = [
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (0, 0, 0),
        (255, 255, 255)
    ];


    for index in 0..COLORS.len() {
        let color = COLORS[index];
        let (red, green, blue) = color;

        println!("INDEX: {index} RGB({red}, {green}, {blue})");

        match get_most_level_color(color) {
            Ok((color_level, color_name)) => {
                println!("{color_name} is the greater level with {color_level}");
            },
            Err(err) => {
                println!("{err}");
            }
        }

        println!("");
    }
}

fn get_most_level_color(color: (u8, u8, u8)) -> Result<(u8, String), String> {
    let (red, green, blue) = color;

    if red > green && red > blue {
        return Ok((red, "red".to_string()));
    } else if green > red && green > blue {
        return Ok((green, "green".to_string()));
    } else if blue > red && blue > green {
        return Ok((blue, "blue".to_string()));
    }

    return Err("Todas as cores tem o mesmo nível".to_string());
}
