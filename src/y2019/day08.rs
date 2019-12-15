use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Color {
    Black,
    White,
    Transparent,
}

impl Color {
    fn from_char(c: char) -> Option<Color> {
        match c {
            '0' => Some(Color::Black),
            '1' => Some(Color::White),
            '2' => Some(Color::Transparent),
            _ => None,
        }
    }

    fn to_pretty_char(&self) -> char {
        match self {
            Color::Black => ' ',
            Color::White => '■',
            Color::Transparent => '_',
        }
    }
}

pub fn answer1() {
    let (width, height) = (25, 6);
    let layer_len = width * height;
    let pixels = read_data();
    assert!(
        pixels.len() % layer_len == 0,
        format!(
            "incomplete layer ({}, {}) with {} total pixels",
            width,
            height,
            pixels.len()
        )
    );
    let number_of_layers = pixels.len() / layer_len;
    let l = (0..number_of_layers)
        .map(|i| &pixels[i * layer_len..(i + 1) * layer_len])
        .min_by(|x, y| number_of(Color::Black, x).cmp(&number_of(Color::Black, y)))
        .unwrap();
    let result = number_of(Color::White, l) * number_of(Color::Transparent, l);
    println!("{}", result);
}

pub fn answer2() {
    let (width, height) = (25, 6);
    let pixels = read_data();
    let final_image = merge_image(width, height, &pixels);

    for y in 0..height {
        for x in 0..width {
            print!("{}", final_image[y * width + x].to_pretty_char());
        }
        print!("\n");
    }
    // RCYKR
}

fn number_of<T: Eq>(target: T, xs: &[T]) -> usize {
    xs.iter().filter(|x| **x == target).count()
}

fn merge_image<'a>(width: usize, height: usize, pixels: &'a Vec<Color>) -> Vec<Color> {
    let layer_len = width * height;
    assert!(
        pixels.len() % layer_len == 0,
        format!(
            "incomplete layer ({}, {}) with {} total pixels",
            width,
            height,
            pixels.len()
        )
    );
    let number_of_layers = pixels.len() / layer_len;
    let layers = (0..number_of_layers)
        .map(|i| &pixels[i * layer_len..(i + 1) * layer_len])
        .collect();
    let mut final_image = Vec::with_capacity(layer_len);
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            final_image.push(pixel_color(&layers, idx).clone());
        }
    }

    final_image
}

fn pixel_color<'a>(layers: &'a Vec<&[Color]>, i: usize) -> &'a Color {
    layers.iter().fold(&Color::Transparent, |current, l| {
        let c = &l[i];
        match current {
            Color::Transparent => c,
            _ => current,
        }
    })
}

fn read_data() -> Vec<Color> {
    fs::read_to_string("data/2019/day08.txt")
        .unwrap()
        .trim()
        .chars()
        .flat_map(Color::from_char)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_layer_color() {
        let pixels = vec![
            Color::Black,
            Color::Transparent,
            Color::Transparent,
            Color::Transparent,
            Color::White,
            Color::White,
            Color::Transparent,
            Color::Transparent,
            Color::Transparent,
            Color::Transparent,
            Color::White,
            Color::Transparent,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
        ];
        assert_eq!(
            merge_image(2, 2, &pixels),
            [Color::Black, Color::White, Color::White, Color::Black]
        );
    }
}
