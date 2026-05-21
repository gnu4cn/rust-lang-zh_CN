use std::process::Command;

fn main() -> std::io::Result<()> {
    // Invoke mdbook build in the directory containing book.toml
    let status = Command::new("mdbook")
        .arg("build")
        .status()?;

    if status.success() {
        println!("PDF successfully generated in book/pdf/output.pdf");
    } else {
        eprintln!("Failed to generate PDF");
    }

    Ok(())
}
