extern crate rusttype;

use rusttype::{FontCollection, PixelsXY, point, PositionedGlyph};
use std::io::Write;

#[test]
fn kerning() {
    //let font_data = include_bytes!("Gudea-Regular.ttf"); // works, but kerning is always 0
    let font_data = include_bytes!("../DejaVuSans.ttf"); // doesn't work

    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap(); // only succeeds if collection consists of one font

    // Desired font pixel height
    let height: f32 = 12.5; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let scale = PixelsXY(height*2.0, height);

    // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
    // We don't want to clip the text, so we shift it down with an offset when laying it out.
    // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Glyphs to draw for "RustType". Feel free to try other strings.
    let mut string = String::new();
    for i in 0..129 {
        let c = (i as u8) as char;
        string.push(c);
    }
    for c in string.chars() {
        for c_other in string.chars() {
            //println!("{:?}, {:?}", c, c_other);
            let kerning = font.pair_kerning(scale, c, c_other);
            if kerning != 0.0 {
                // kerning is always 0.0 so nothing is printed
                println!("kerning {} {} {:.10} ", c, c_other, kerning);
            }
        }
    }
}
