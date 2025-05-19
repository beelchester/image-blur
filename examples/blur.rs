use image::{DynamicImage, ImageReader, RgbImage};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("test.jpg")?.decode()?;
    let buf = img.to_rgb8();
    let width = img.width();
    let height = img.height();
    let bytes = buf.clone().into_vec();
    let image = Image {
        width,
        height,
        data: bytes,
    };

    let url = "http://localhost:8080/blur";
    let client = reqwest::blocking::Client::new();

    let res = client.post(url).json(&image).send();
    let res = res.map_err(|e| dbg!(e)).unwrap();
    let json: Image = res.json()?;

    let bytes = json.data;
    let rgb_image_buf = RgbImage::from_vec(json.width, json.height, bytes).unwrap();

    let dyn_img = DynamicImage::ImageRgb8(rgb_image_buf);
    dyn_img.save("finally.jpg")?;

    Ok(())
}
