use std::{fs::File, io::Write};

use axum::{
    extract::{ContentLengthLimit, Multipart},
    response::Html,
};
use walkdir::WalkDir;

pub async fn ping() -> String {
    "Pong".to_string()
}

pub async fn get_files() -> Html<String> {
    let mut html = "".to_string();
    for files in WalkDir::new("uploads/") {
        let file_name = &files.as_ref().unwrap().path().display();
        if files.as_ref().unwrap().metadata().unwrap().is_file() {
            html = format!(
                "{}\n<li><a href={}>/{}</a></li>",
                html, file_name, file_name
            );
        }
    }
    Html(format!(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>
                    <input type="submit" value="Upload files">
                </form>
                <h3> Files In Directory</h3>
                <br>
                <ul>
                {}
                </ul>
            </body>
        </html>
        "#,
        html
    ))
}

pub async fn accept_form(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let mut file = File::create("uploads/".to_string() + &file_name).unwrap();
        file.write_all(&data).unwrap();
    }
}
