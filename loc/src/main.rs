use std::time::Instant;
use loc::{get_language_name, Files};

fn main() {
    let start = Instant::now();
    let counts = Files::new(None); // If we dont pass a path it will use the current_dir path as default
    let end = start.elapsed();

    println!("{}", "-".repeat(85));
    println!(
        "{:<15} {:>12} {:>18} {:>16} {:>12}",
        "Language", "Files", "Blank Spaces", "Lines of Code", "Comments"
    );
    println!("{}", "-".repeat(85));

    for count in counts.counts {
        println!(
            "{:<15} {:>12} {:>18} {:>16} {:>12}",
            get_language_name(&count.0),
            count.1.nums_of_files,
            count.1.blank_spaces_count,
            count.1.lines_of_code,
            count.1.comments_count,
        );
    }
    println!("Reading all file done in {:?}", end);
}
