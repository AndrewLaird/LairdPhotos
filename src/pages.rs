use askama::Template;
// 1. Define a stcruct that implements the Template trait
#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate {
}

pub fn upload() -> String {
    let upload = UploadTemplate {
    };
    return upload.render().unwrap();
}
