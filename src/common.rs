use std::io;
use std::path::Path;
use std::fs::File;
use reqwest::multipart::Part;
use reqwest::multipart::Body;
use mime_guess::Mime;


pub fn file_to_part<T: AsRef<Path>>(path: T) -> io::Result<Part> {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .map(|filename| filename.to_string_lossy().into_owned());
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let mime = mime_guess::from_ext(ext).first_or_octet_stream();
    let file = File::open(path)?;
    let field = Part::new(Body::from(file)).mime(mime);

    Ok(if let Some(file_name) = file_name {
        field.file_name(file_name)
    } else {
        field
    })
}