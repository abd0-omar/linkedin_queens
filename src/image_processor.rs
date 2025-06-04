use image::{DynamicImage, GenericImageView, Rgba};
use std::error::Error;

use crate::CellColor;

// const COLOR_TOLERANCE: u8 = 20;
// const BLACK_THRESHOLD: u8 = 30; // Increased threshold for black detection
// const MIN_LINE_LENGTH: u32 = 10; // Minimum length of a line to be considered a grid line
// const LINE_DETECTION_THRESHOLD: u32 = 2; // Reduced from 3 to 2
const MIN_GRID_SIZE: u32 = 6; // Minimum expected grid size
const WINDOW_SIZE: u32 = 5; // Size of sliding window for line detection

// Add color constants at module level
const COLOR_DEFINITIONS: &[((u8, u8, u8), CellColor)] = &[
    ((241, 203, 154), CellColor::PeachOrange),
    ((229, 130, 104), CellColor::VibrantCoral),
    ((164, 190, 249), CellColor::SoftBlue),
    ((190, 221, 166), CellColor::PastelGreen),
    ((223, 223, 223), CellColor::LightGray),
    ((183, 165, 221), CellColor::Lavender),
    ((231, 242, 151), CellColor::LimeYellow),
    ((183, 178, 160), CellColor::DarkGray),
    ((209, 163, 190), CellColor::Pink),
    ((241, 234, 218), CellColor::WarmBeige),
];

#[derive(Debug)]
pub struct BoardImage {
    image: DynamicImage,
    cell_width: u32,
    cell_height: u32,
    grid_width: u32,
    grid_height: u32,
}

impl BoardImage {
    pub fn new(image: DynamicImage) -> Result<Self, Box<dyn Error>> {
        let (width, height) = image.dimensions();
        println!("Image dimensions: {}x{}", width, height);

        // Detect lines using the consolidated function
        let horizontal_lines = detect_lines(&image, true);
        dbg!(&horizontal_lines);
        let vertical_lines = detect_lines(&image, false);
        dbg!(&vertical_lines);

        println!(
            "Found {} horizontal lines and {} vertical lines",
            horizontal_lines.len(),
            vertical_lines.len()
        );

        // Remove any lines that are too close together
        // horizontal_lines = remove_close_lines(&horizontal_lines, height / 20);
        // vertical_lines = remove_close_lines(&vertical_lines, width / 20);
        // horizontal_lines = remove_close_lines(&horizontal_lines, height / 10);
        // vertical_lines = remove_close_lines(&vertical_lines, width / 10);

        println!(
            "After filtering: {} horizontal lines and {} vertical lines",
            horizontal_lines.len(),
            vertical_lines.len()
        );

        if horizontal_lines.len() < MIN_GRID_SIZE as usize
            || vertical_lines.len() < MIN_GRID_SIZE as usize
        {
            return Err(format!(
                "Could not detect enough grid lines. Found {} horizontal and {} vertical lines",
                horizontal_lines.len(),
                vertical_lines.len()
            )
            .into());
        }

        // Calculate cell dimensions from the gaps between lines
        let cell_height = calculate_average_gap(&horizontal_lines);
        let cell_width = calculate_average_gap(&vertical_lines);

        println!("Detected cell dimensions: {}x{}", cell_width, cell_height);

        if cell_height == 0 || cell_width == 0 {
            return Err("Invalid cell dimensions detected".into());
        }

        // Calculate grid dimensions
        let grid_height = horizontal_lines.len() - 1;
        let grid_width = vertical_lines.len() - 1;

        println!("Detected grid dimensions: {}x{}", grid_width, grid_height);

        // Verify that width and height are equal (n*n board)
        if grid_width != grid_height {
            return Err(format!(
                "Board must be square (n*n), got {}x{}",
                grid_width, grid_height
            )
            .into());
        }

        Ok(Self {
            image,
            cell_width,
            cell_height,
            grid_width: grid_width as u32,
            grid_height: grid_height as u32,
        })
    }

    pub fn get_board_colors(&self) -> Vec<Vec<CellColor>> {
        let mut board =
            vec![vec![CellColor::LightGray; self.grid_width as usize]; self.grid_height as usize];

        for row in 0..self.grid_height {
            for col in 0..self.grid_width {
                let cell_colors = self.sample_cell_colors(row, col);
                // difference of colors is about 14 diff at R
                board[row as usize][col as usize] = detect_dominant_color(&cell_colors);
            }
        }

        board
    }

    fn sample_cell_colors(&self, row: u32, col: u32) -> Vec<Rgba<u8>> {
        let mut colors = Vec::new();
        // the +1 is a small offset to avoid sampling the border of the cell
        // to ensure we are sampling inside the cell
        let cell_x = 1 + col * (self.cell_width + 1);
        let cell_y = 1 + row * (self.cell_height + 1);

        // let x_steps = 5;
        // let y_steps = 5;
        // let x_step = self.cell_width / x_steps;
        // let y_step = self.cell_height / y_steps;

        // // Sample grid points
        // for y_offset in 1..y_steps {
        //     for x_offset in 1..x_steps {
        //         let x = cell_x + x_offset * x_step;
        //         let y = cell_y + y_offset * y_step;
        //         if let Some(pixel) = sample_pixel(&self.image, x, y) {
        //             colors.push(pixel);
        //         }
        //     }
        // }

        // // Sample corners if needed
        // if colors.len() < 4 {
        //     let corners = [
        //         (cell_x + x_step, cell_y + y_step),
        //         (cell_x + self.cell_width - x_step, cell_y + y_step),
        //         (cell_x + x_step, cell_y + self.cell_height - y_step),
        //         (
        //             cell_x + self.cell_width - x_step,
        //             cell_y + self.cell_height - y_step,
        //         ),
        //     ];

        //     for (x, y) in corners {
        //         if let Some(pixel) = sample_pixel(&self.image, x, y) {
        //             colors.push(pixel);
        //         }
        //     }
        // }

        // dbg!("not enough color samples");

        // Sample center if still empty
        // if colors.is_empty() {
        // center of the bottom left corner of the cell
        // if let Some(pixel) = sample_pixel(&self.image, center_x, center_y) {
        //     colors.push(pixel);
        // }
        let center_x = cell_x + self.cell_width / 4; // 1/4 from left
        let center_y = cell_y + (self.cell_height * 3) / 4; // 1/4 from bottom
        colors.push(sample_pixel(&self.image, center_x, center_y));
        // }

        colors
    }
}

// if the image is low quality, the pixels are not black, but a dark gray
// fn is_dark(pixel: Rgba<u8>) -> bool {
//     pixel[0] < BLACK_THRESHOLD && pixel[1] < BLACK_THRESHOLD && pixel[2] < BLACK_THRESHOLD
// }

fn is_black(pixel: Rgba<u8>) -> bool {
    pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0
}

// fn is_white(pixel: Rgba<u8>) -> bool {
//     pixel[0] > 240 && pixel[1] > 240 && pixel[2] > 240
// }

// fn color_matches(pixel: Rgba<u8>, target: (u8, u8, u8)) -> bool {
//     let (r, g, b) = target;
//     (pixel[0] as i16 - r as i16).abs() <= COLOR_TOLERANCE as i16
//         && (pixel[1] as i16 - g as i16).abs() <= COLOR_TOLERANCE as i16
//         && (pixel[2] as i16 - b as i16).abs() <= COLOR_TOLERANCE as i16
// }

fn detect_dominant_color(colors: &[Rgba<u8>]) -> CellColor {
    // if colors.is_empty() {
    //     return CellColor::LightGray;
    // }

    // Count occurrences of each color
    // let mut color_counts = std::collections::HashMap::new();
    // for &pixel in colors {
    //     let color = detect_single_color(pixel).unwrap();
    //     *color_counts.entry(color).or_insert(0) += 1;
    // }
    // dbg!(&color_counts);

    // // Find the most common color
    // color_counts
    //     .into_iter()
    //     .max_by_key(|&(_, count)| count)
    //     .map(|(color, _)| color)
    //     .unwrap_or(CellColor::LightGray)
    // dbg!(&colors[0]);
    detect_single_color(colors[0]).unwrap()
}

fn detect_single_color(pixel: Rgba<u8>) -> Result<CellColor, String> {
    // if is_white(pixel) {
    //     return CellColor::LightGray;
    // }

    let mut closest_color = CellColor::LightGray;
    // min_diff will decrease as we go through the loop, till we find the closest color
    let mut min_diff = u32::MAX;

    for ((r, g, b), color) in COLOR_DEFINITIONS {
        let diff = ((pixel[0] as i32 - *r as i32).abs()
            + (pixel[1] as i32 - *g as i32).abs()
            + (pixel[2] as i32 - *b as i32).abs()) as u32;
        if diff < min_diff {
            min_diff = diff;
            closest_color = *color;
        }
    }

    if min_diff > 100 {
        Err(format!("Color difference too large: {}", min_diff))
    } else {
        Ok(closest_color)
    }
}

// fn remove_close_lines(lines: &[u32], min_distance: u32) -> Vec<u32> {
//     if lines.is_empty() {
//         return Vec::new();
//     }

//     let mut result = Vec::new();
//     let mut last_line = lines[0];
//     result.push(last_line);

//     for &line in &lines[1..] {
//         if line - last_line >= min_distance {
//             result.push(line);
//             last_line = line;
//         }
//     }

//     result
// }

fn calculate_average_gap(lines: &[u32]) -> u32 {
    let mut gaps = Vec::new();
    for window in lines.windows(2) {
        gaps.push(window[1] - window[0]);
    }

    gaps.iter().sum::<u32>() / gaps.len() as u32
}

// Helper function for line detection
fn detect_lines(image: &DynamicImage, is_horizontal: bool) -> Vec<u32> {
    let (width, height) = image.dimensions();
    let mut lines = Vec::new();
    let mut window = vec![0; WINDOW_SIZE as usize];
    let mut window_index = 0;
    let mut in_line = false;
    let mut line_start = 0;

    let (primary_dim, secondary_dim) = if is_horizontal {
        (height, width)
    } else {
        (width, height)
    };

    for p in 0..primary_dim {
        let mut dark_count = 0;
        for s in 0..secondary_dim {
            let (x, y) = if is_horizontal { (s, p) } else { (p, s) };
            // if is_dark(image.get_pixel(x, y)) {
            //     dark_count += 1;
            // }
            if is_black(image.get_pixel(x, y)) {
                dark_count += 1;
            }
        }

        window[window_index] = dark_count;
        window_index = (window_index + 1) % WINDOW_SIZE as usize;

        let window_avg: u32 = window.iter().sum::<u32>() / WINDOW_SIZE;
        // to determine when a line starts
        // if the number of black pixels is greater than 1/4 of the secondary dimension, then a line starts
        let threshold = secondary_dim as u32 / 4;
        // to determine when a line ends
        // if the number of black pixels is less than 1/8 of the secondary dimension, then a line ends
        let end_threshold = secondary_dim as u32 / 8;

        if window_avg > threshold && !in_line {
            in_line = true;
            line_start = p;
        } else if window_avg <= end_threshold && in_line {
            in_line = false;
            let line_middle = line_start + (p - line_start) / 2;
            lines.push(line_middle);
        }
    }

    lines
}

// Helper function for sampling a single pixel
fn sample_pixel(image: &DynamicImage, x: u32, y: u32) -> Rgba<u8> {
    let pixel = image.get_pixel(x, y);
    // I think we are sure it won't be black or white, so we can just sample the pixel
    // if !is_black(pixel) && !is_white(pixel) {
    //     Some(pixel)
    // } else {
    //     None
    // }
    pixel
}

pub fn process_image(image_path: &str) -> Result<Vec<Vec<CellColor>>, Box<dyn Error>> {
    let image = image::open(image_path)?;
    let board_image = BoardImage::new(image)?;
    Ok(board_image.get_board_colors())
}
