use reqwest::blocking::{multipart, Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("no file specified");
            "https://hyper.rs".into()
        }
    };

    let file_data = std::fs::read(&path)?;
    let file_name = path.split('/').last().unwrap_or("upload");

    let form = multipart::Form::new().part(
        "file",
        multipart::Part::bytes(file_data)
            .file_name(file_name.to_string())
            .mime_str("image/png")?,
    );

    let mut response = Client::new()
        .post("https://u.rj1.su/upload")
        .multipart(form)
        .send()?;

    response.copy_to(&mut std::io::stdout())?;

    Ok(())
}
