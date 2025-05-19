use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use image::{DynamicImage, RgbImage};

// jpg image -> image bytes -> server recieves data -> create dyn_img -> apply blur -> send back
// the rgb8 bytes

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

async fn blur_img(json: web::Json<Image>) -> impl Responder {
    let json = json.into_inner();
    let bytes = json.data;
    let rgb_image_buf = RgbImage::from_vec(json.width, json.height, bytes).unwrap();

    let mut dyn_img = DynamicImage::ImageRgb8(rgb_image_buf);
    dyn_img = dyn_img.fast_blur(10.0);
    let new_bytes = dyn_img.to_rgb8().into_vec();
    let blurred_image = Image {
        data: new_bytes,
        width: dyn_img.width(),
        height: dyn_img.height(),
    };
    let json_str = serde_json::to_string(&blurred_image).unwrap();
    HttpResponse::Ok().body(json_str)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/blur", web::post().to(blur_img)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
