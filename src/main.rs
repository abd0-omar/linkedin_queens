use linkedin_queens::{process_image, queens};

fn main() {
    // Example: Process board from image
    match process_image("image_04062025.png") {
        Ok(board) => {
            println!("Board from image:");
            for row in &board {
                println!("{:?}", row);
            }
            // Try to solve the queens puzzle
            if let Err(e) = queens(&board) {
                println!("Error solving puzzle: {}", e);
            }
        }
        Err(e) => println!("Error processing image: {}", e),
    }
}
