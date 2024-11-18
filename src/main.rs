use clap::Parser;
use reqwest::blocking::{multipart, Client};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let form = multipart::Form::new().part(
        "file",
        multipart::Part::bytes(std::fs::read(&cli.path).expect("could not read file"))
            .file_name(
                cli.path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or("invalid or missing file name")?
                    .to_string(),
            )
            .mime_str(
                mime_guess::from_path(&cli.path)
                    .first_or_octet_stream()
                    .as_ref(),
            )?,
    );

    let mut response = Client::new()
        .post("https://u.rj1.su/upload")
        .multipart(form)
        .send()?;

    response.copy_to(&mut std::io::stdout())?;

    Ok(())
}
