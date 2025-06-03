use image::{DynamicImage, GenericImageView, Rgba};
use std::error::Error;

use crate::CellColor;

const COLOR_TOLERANCE: u8 = 20;
const BLACK_THRESHOLD: u8 = 30; // Increased threshold for black detection
const MIN_LINE_LENGTH: u32 = 10; // Minimum length of a line to be considered a grid line
const LINE_DETECTION_THRESHOLD: u32 = 2; // Reduced from 3 to 2
const MIN_GRID_SIZE: u32 = 6; // Minimum expected grid size
const WINDOW_SIZE: u32 = 5; // Size of sliding window for line detection

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

        // First, try to detect the grid size by looking for repeating patterns
        let mut horizontal_lines = Vec::new();
        let mut vertical_lines = Vec::new();

        // Detect horizontal lines using a sliding window
        let mut window = vec![0; WINDOW_SIZE as usize];
        let mut window_index = 0;
        let mut in_line = false;
        let mut line_start = 0;

        for y in 0..height {
            // Count dark pixels in this row
            let mut dark_count = 0;
            for x in 0..width {
                if is_dark(image.get_pixel(x, y)) {
                    dark_count += 1;
                }
            }

            // Update sliding window
            window[window_index] = dark_count;
            window_index = (window_index + 1) % WINDOW_SIZE as usize;

            // Calculate average dark pixels in window
            let window_avg: u32 = window.iter().sum::<u32>() / WINDOW_SIZE;

            // Detect line transitions
            if window_avg > width as u32 / 4 && !in_line {
                // Start of a line
                in_line = true;
                line_start = y;
            } else if window_avg <= width as u32 / 8 && in_line {
                // End of a line
                in_line = false;
                let line_middle = line_start + (y - line_start) / 2;
                horizontal_lines.push(line_middle);
            }
        }

        // Reset for vertical lines
        window = vec![0; WINDOW_SIZE as usize];
        window_index = 0;
        in_line = false;
        line_start = 0;

        for x in 0..width {
            // Count dark pixels in this column
            let mut dark_count = 0;
            for y in 0..height {
                if is_dark(image.get_pixel(x, y)) {
                    dark_count += 1;
                }
            }

            // Update sliding window
            window[window_index] = dark_count;
            window_index = (window_index + 1) % WINDOW_SIZE as usize;

            // Calculate average dark pixels in window
            let window_avg: u32 = window.iter().sum::<u32>() / WINDOW_SIZE;

            // Detect line transitions
            if window_avg > height as u32 / 4 && !in_line {
                // Start of a line
                in_line = true;
                line_start = x;
            } else if window_avg <= height as u32 / 8 && in_line {
                // End of a line
                in_line = false;
                let line_middle = line_start + (x - line_start) / 2;
                vertical_lines.push(line_middle);
            }
        }

        println!(
            "Found {} horizontal lines and {} vertical lines",
            horizontal_lines.len(),
            vertical_lines.len()
        );

        // Sort the lines to ensure they're in order
        // horizontal_lines.sort();
        // vertical_lines.sort();

        // Remove any lines that are too close together
        horizontal_lines = remove_close_lines(&horizontal_lines, height / 20);
        vertical_lines = remove_close_lines(&vertical_lines, width / 20);
        // horizontal_lines = remove_close_lines(&horizontal_lines, height / 10);
        // vertical_lines = remove_close_lines(&vertical_lines, width / 10);

        println!(
            "After filtering: {} horizontal lines and {} vertical lines",
            horizontal_lines.len(),
            vertical_lines.len()
        );

        if horizontal_lines.len() < 2 || vertical_lines.len() < 2 {
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

        // Verify that we have a valid grid
        if grid_width < MIN_GRID_SIZE as usize || grid_height < MIN_GRID_SIZE as usize {
            return Err(format!(
                "Grid too small: {}x{}. Expected at least {}x{}",
                grid_width, grid_height, MIN_GRID_SIZE, MIN_GRID_SIZE
            )
            .into());
        }

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
                board[row as usize][col as usize] = detect_dominant_color(&cell_colors);
            }
        }

        board
    }

    fn sample_cell_colors(&self, row: u32, col: u32) -> Vec<Rgba<u8>> {
        let mut colors = Vec::new();
        let cell_x = 1 + col * (self.cell_width + 1);
        let cell_y = 1 + row * (self.cell_height + 1);

        // Sample more points in a denser grid pattern
        let x_steps = 5; // Increased from 3 to 5
        let y_steps = 5; // Increased from 3 to 5
        let x_step = self.cell_width / x_steps;
        let y_step = self.cell_height / y_steps;

        // Sample points in a grid pattern, avoiding the edges
        for y_offset in 1..y_steps {
            for x_offset in 1..x_steps {
                let x = cell_x + x_offset * x_step;
                let y = cell_y + y_offset * y_step;
                let pixel = self.image.get_pixel(x, y);

                // Only collect non-black colors and non-white colors
                if !is_black(pixel) && !is_white(pixel) {
                    colors.push(pixel);
                }
            }
        }

        // If we didn't get enough samples, try the corners
        if colors.len() < 4 {
            let corners = [
                (cell_x + x_step, cell_y + y_step),                    // top-left
                (cell_x + self.cell_width - x_step, cell_y + y_step),  // top-right
                (cell_x + x_step, cell_y + self.cell_height - y_step), // bottom-left
                (
                    cell_x + self.cell_width - x_step,
                    cell_y + self.cell_height - y_step,
                ), // bottom-right
            ];

            for (x, y) in corners {
                let pixel = self.image.get_pixel(x, y);
                if !is_black(pixel) && !is_white(pixel) {
                    colors.push(pixel);
                }
            }
        }

        // If still no colors found, use the center
        if colors.is_empty() {
            let center_x = cell_x + self.cell_width / 2;
            let center_y = cell_y + self.cell_height / 2;
            let pixel = self.image.get_pixel(center_x, center_y);
            if !is_black(pixel) {
                colors.push(pixel);
            }
        }

        colors
    }

    pub fn get_grid_size(&self) -> u32 {
        self.grid_width // Since we verified width == height, either is fine
    }
}

fn is_dark(pixel: Rgba<u8>) -> bool {
    pixel[0] < BLACK_THRESHOLD && pixel[1] < BLACK_THRESHOLD && pixel[2] < BLACK_THRESHOLD
}

fn is_black(pixel: Rgba<u8>) -> bool {
    pixel[0] < 10 && pixel[1] < 10 && pixel[2] < 10
}

fn is_white(pixel: Rgba<u8>) -> bool {
    pixel[0] > 240 && pixel[1] > 240 && pixel[2] > 240
}

fn color_matches(pixel: Rgba<u8>, target: (u8, u8, u8)) -> bool {
    let (r, g, b) = target;
    (pixel[0] as i16 - r as i16).abs() <= COLOR_TOLERANCE as i16
        && (pixel[1] as i16 - g as i16).abs() <= COLOR_TOLERANCE as i16
        && (pixel[2] as i16 - b as i16).abs() <= COLOR_TOLERANCE as i16
}

fn detect_dominant_color(colors: &[Rgba<u8>]) -> CellColor {
    if colors.is_empty() {
        return CellColor::LightGray;
    }

    // Count occurrences of each color
    let mut color_counts = std::collections::HashMap::new();
    for &pixel in colors {
        let color = detect_single_color(pixel);
        *color_counts.entry(color).or_insert(0) += 1;
    }

    // Find the most common color
    color_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(color, _)| color)
        .unwrap_or(CellColor::LightGray)
}

fn detect_single_color(pixel: Rgba<u8>) -> CellColor {
    if is_white(pixel) {
        return CellColor::LightGray;
    }

    // Try exact matches first
    match pixel {
        p if p[0] == 241 && p[1] == 203 && p[2] == 154 => CellColor::PeachOrange,
        p if p[0] == 229 && p[1] == 130 && p[2] == 104 => CellColor::VibrantCoral,
        p if p[0] == 164 && p[1] == 190 && p[2] == 249 => CellColor::SoftBlue,
        p if p[0] == 190 && p[1] == 221 && p[2] == 166 => CellColor::PastelGreen,
        p if p[0] == 223 && p[1] == 223 && p[2] == 223 => CellColor::LightGray,
        p if p[0] == 183 && p[1] == 165 && p[2] == 221 => CellColor::Lavender,
        p if p[0] == 231 && p[1] == 242 && p[2] == 151 => CellColor::LimeYellow,
        p if p[0] == 183 && p[1] == 178 && p[2] == 160 => CellColor::DarkGray,
        p if p[0] == 209 && p[1] == 163 && p[2] == 190 => CellColor::Pink,
        p if p[0] == 241 && p[1] == 234 && p[2] == 218 => CellColor::WarmBeige,
        // If no exact match, try with tolerance
        p => {
            let colors = [
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

            let mut closest_color = CellColor::LightGray;
            let mut min_diff = u32::MAX;

            for ((r, g, b), color) in colors {
                let diff = ((p[0] as i32 - r as i32).abs()
                    + (p[1] as i32 - g as i32).abs()
                    + (p[2] as i32 - b as i32).abs()) as u32;
                if diff < min_diff {
                    min_diff = diff;
                    closest_color = color;
                }
            }

            if min_diff > 100 {
                CellColor::LightGray
            } else {
                closest_color
            }
        }
    }
}

fn remove_close_lines(lines: &[u32], min_distance: u32) -> Vec<u32> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut last_line = lines[0];
    result.push(last_line);

    for &line in &lines[1..] {
        if line - last_line >= min_distance {
            result.push(line);
            last_line = line;
        }
    }

    result
}

fn calculate_average_gap(lines: &[u32]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }

    let mut gaps = Vec::new();
    for window in lines.windows(2) {
        gaps.push(window[1] - window[0]);
    }

    gaps.iter().sum::<u32>() / gaps.len() as u32
}

pub fn process_image(image_path: &str) -> Result<Vec<Vec<CellColor>>, Box<dyn Error>> {
    let image = image::open(image_path)?;
    let board_image = BoardImage::new(image)?;
    Ok(board_image.get_board_colors())
}
