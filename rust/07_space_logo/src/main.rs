use std::error::Error;
use image::{Rgb, RgbImage, ImageBuffer};
use imageproc::drawing::{draw_cross_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use show_image::{event, ImageView, ImageInfo, create_window, Image};

const WHITE: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);

const LOGO_W: i32 = 40; // width of the logo square frame
const LOGO_TL: [i32; 2] = [27, 27]; // starting point of the logo

const DOT_W: i32 = 8; // the width of the inner dot (dot is actually an square
const DOT_TL: [i32; 2] = [36, 36]; // starting point of the inner dot

const FONT_H: i32 = 15; // font height
const FONT_W: i32 = 12; // font width
const FONT_B: i32 = 3;  // font bold
const LSP: i32 = 3;     // letter space

const LOGO_BG: [[i32;2]; 2] = [[20, 20], [154, 74]];

fn draw_logo_background(pixel_data: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for i in LOGO_BG[0][0]..LOGO_BG[1][0]  {
        for j in LOGO_BG[0][1]..LOGO_BG[1][1]  {
            println!("{}, {}", i, j);
            draw_cross_mut(pixel_data, WHITE, i, j);
        }
    }
}

fn draw_logo_frame(pixel_data: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for i in LOGO_TL[0]..(LOGO_TL[0] + LOGO_W) {
        for j in LOGO_TL[1]..(LOGO_TL[1] + LOGO_W) {
            if i > LOGO_TL[0] + 2
                && j > LOGO_TL[1] + 2
                && i < LOGO_TL[0] + LOGO_W - 3
                && j < LOGO_TL[1] + LOGO_W - 3 {
                continue;
            }
            println!("{}, {}", i, j);
            draw_cross_mut(pixel_data, BLACK, i, j);
        }
    }
}

fn draw_dot(pixel_data: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for i in DOT_TL[0]..(DOT_TL[0] + DOT_W) {
        for j in DOT_TL[1]..(DOT_TL[1] + DOT_W) {
            println!("{}, {}", i, j);
            draw_cross_mut(pixel_data, BLACK, i, j);
        }
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let mut pixel_data = image::open(
        "/Users/charliechen/Dropbox/code/matters/utilities/space/thespace_bg.png"
    ).unwrap().to_rgb8();

    draw_logo_background(&mut pixel_data);
    draw_logo_frame(&mut pixel_data);
    draw_dot(&mut pixel_data);

    let image = ImageView::new(ImageInfo::rgb8(1000, 1000), &pixel_data);
    let window = create_window("TheSpace", Default::default())?;

    window.set_image("TheSpace", image)?;
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
    Ok(())
}
