use askama::Template;
// 1. Define a stcruct that implements the Template trait
#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate {
    name: String
}

pub fn upload(name: String) -> String {
    let upload = UploadTemplate {
        name
    };
    return upload.render().unwrap();
}
