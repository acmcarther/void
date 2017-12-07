#![deny(warnings)]

extern crate heatmap;
extern crate hsl;
extern crate png;
extern crate rusttype;

use heatmap::Heatmap;
use hsl::HSL;
use png::HasParameters;
use rusttype::{FontCollection, PixelsXY, PositionedGlyph, point};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const US: u64 = 1_000;
const MS: u64 = 1_000 * US;

#[allow(dead_code)]
pub struct Waterfall {
    config: Config,
}

pub struct Config {}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    pub fn build(self) -> Waterfall {
        Waterfall::configured(self)
    }
}

struct Label {
    value: u64,
    text: String,
}

impl Default for Waterfall {
    fn default() -> Waterfall {
        Waterfall::configured(Config::default())
    }
}

impl Waterfall {
    pub fn new() -> Waterfall {
        Default::default()
    }

    pub fn configure() -> Config {
        Config::default()
    }

    fn configured(config: Config) -> Waterfall {
        Waterfall { config: config }
    }

    pub fn render_png(&mut self, heatmap: &Heatmap, file: String) {
        let height = heatmap.num_slices() as usize;
        let width = heatmap.histogram_buckets() as usize;

        // build buffer from data
        let mut buffer = ImageBuffer::<ColorRgb>::new(width, height);
        let max = find_max(heatmap);
        let mut x;
        let mut y = 0;

        // loop to color the pixels
        for slice in heatmap {
            x = 0;
            for bucket in &slice.histogram() {
                let value = (bucket.count() as f64) / (bucket.width() as f64);
                let pixel = color_from_value(value, max);
                buffer.set_pixel(x, y, pixel);
                x += 1;
            }
            y += 1;
        }

        // latency annotations
        let labels: Vec<Label> = vec![
            Label {
                value: 200,
                text: "200nS".to_string(),
            },
            Label {
                value: 500,
                text: "500nS".to_string(),
            },
            Label {
                value: US,
                text: "1uS".to_string(),
            },
            Label {
                value: 2 * US,
                text: "2uS".to_string(),
            },
            Label {
                value: 5 * US,
                text: "5uS".to_string(),
            },
            Label {
                value: 10 * US,
                text: "10uS".to_string(),
            },
            Label {
                value: 20 * US,
                text: "20uS".to_string(),
            },
            Label {
                value: 50 * US,
                text: "50uS".to_string(),
            },
            Label {
                value: 100 * US,
                text: "100uS".to_string(),
            },
            Label {
                value: 200 * US,
                text: "200uS".to_string(),
            },
            Label {
                value: 500 * US,
                text: "500uS".to_string(),
            },
            Label {
                value: MS,
                text: "1mS".to_string(),
            },
            Label {
                value: 2 * MS,
                text: "2mS".to_string(),
            },
            Label {
                value: 5 * MS,
                text: "5mS".to_string(),
            },
            Label {
                value: 10 * MS,
                text: "10mS".to_string(),
            },
            Label {
                value: 20 * MS,
                text: "20mS".to_string(),
            },
            Label {
                value: 50 * MS,
                text: "50mS".to_string(),
            },
            Label {
                value: 100 * MS,
                text: "100mS".to_string(),
            },
            Label {
                value: 200 * MS,
                text: "200mS".to_string(),
            },
            Label {
                value: 500 * MS,
                text: "500mS".to_string(),
            },
        ];

        let mut l = 0;
        y = 0;

        for slice in heatmap {
            x = 0;
            for bucket in &slice.histogram() {
                if (y % 60) == 0 {
                    if x == 0 {
                        let hour = y / 3600;
                        let minute = y / 60;
                        let time = format!("{:02}:{:02}", hour, minute);
                        let overlay = string_buffer(&time, 25.0);
                        buffer.overlay(&overlay, x, y);
                        buffer.horizontal_line(y, ColorRgb { r: 0, g: 0, b: 0 });
                    }
                    let v = bucket.value();
                    if (l < labels.len()) && (v >= labels[l].value) {
                        let overlay = string_buffer(&labels[l].text, 25.0);
                        buffer.overlay(&overlay, x, y);
                        buffer.vertical_line(x, ColorRgb { r: 0, g: 0, b: 0 });
                        l += 1;
                    }
                }
                x += 1;
            }
            y += 1;
        }

        let _ = buffer.write_png(file.clone());
    }
}

fn find_max(heatmap: &Heatmap) -> f64 {
    let mut max = 0.0;

    for slice in heatmap {
        for bucket in &slice.histogram() {
            let value = (bucket.count() as f64) / (bucket.width() as f64);
            if value > max {
                max = value;
            }
        }
    }
    max
}

fn string_buffer(string: &str, size: f32) -> ImageBuffer<ColorRgb> {
    // load font
    let font_data = include_bytes!("../assets/ubuntumono/UbuntuMono-Regular.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap();

    // size and scaling
    let height: f32 = size;
    let pixel_height = height.ceil() as usize;
    let scale = PixelsXY(height * 1.0, height);

    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let glyphs: Vec<PositionedGlyph> = font.layout(string, scale, offset).collect();

    let width = glyphs
        .iter()
        .map(|g| g.h_metrics().advance_width)
        .fold(0.0, |x, y| x + y)
        .ceil() as usize;

    let mut overlay = ImageBuffer::<ColorRgb>::new(width, pixel_height);

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = (x as i32 + bb.min.x) as usize;
                let y = (y as i32 + bb.min.y) as usize;
                if v > 0.25 {
                    overlay.set_pixel(
                        x,
                        y,
                        ColorRgb {
                            r: 255,
                            g: 255,
                            b: 255,
                        },
                    );
                }
            })
        }
    }

    overlay
}

fn color_from_value(value: f64, max: f64) -> ColorRgb {
    let value = value / max;

    let knee = 0.10_f64;

    let hsl = if value < knee {
        let l = 0.25_f64 + 0.25_f64 * value / knee;
        HSL {
            h: 236_f64,
            s: 1_f64,
            l: l,
        }
    } else {
        let h_per_deg: f64 = 236_f64 / (1.0_f64 - knee);
        let deg = (value - knee) * h_per_deg;

        HSL {
            h: (236_f64 - deg),
            s: 1_f64,
            l: 0.50_f64,
        }
    };

    let (r, g, b) = hsl.to_rgb();

    ColorRgb { r: r, g: g, b: b }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ColorRgb {
    r: u8,
    g: u8,
    b: u8,
}

struct ImageBuffer<T> {
    buffer: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl ImageBuffer<ColorRgb> {
    pub fn new(width: usize, height: usize) -> ImageBuffer<ColorRgb> {
        let background = ColorRgb { r: 0, g: 0, b: 0 };
        let mut row = Vec::<ColorRgb>::with_capacity(width);
        for _ in 0..width {
            row.push(background);
        }
        let mut buffer = Vec::<Vec<ColorRgb>>::with_capacity(height);
        for _ in 0..height {
            buffer.push(row.clone());
        }
        ImageBuffer {
            buffer: buffer,
            height: height,
            width: width,
        }
    }

    pub fn write_png(self, file: String) -> Result<(), &'static str> {
        let mut buffer = Vec::<u8>::with_capacity(self.height * self.width);
        for row in 0..self.height {
            for col in 0..self.width {
                let pixel = self.buffer[row][col];
                buffer.push(pixel.r);
                buffer.push(pixel.g);
                buffer.push(pixel.b);
            }
        }
        let path = &Path::new(&file);
        if let Ok(file) = File::create(path) {
            let w = BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
            encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
            if let Ok(mut writer) = encoder.write_header() {
                if writer.write_image_data(&buffer).is_ok() {
                    Ok(())
                } else {
                    Err("Error writing PNG data")
                }
            } else {
                Err("Error writing PNG header")
            }
        } else {
            Err("Error creating file")
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: ColorRgb) {
        if x < self.width && y < self.height {
            self.buffer[y][x] = value;
        }
    }

    pub fn overlay(&mut self, other: &ImageBuffer<ColorRgb>, x: usize, y: usize) {
        let ignore = ColorRgb { r: 0, g: 0, b: 0 };
        for sx in 0..other.width {
            for sy in 0..other.height {
                if (other.buffer[sy][sx] != ignore) &&
                    (((sy + y) < self.height) && ((sx + x) < self.width))
                {
                    self.buffer[(sy + y)][(sx + x)] = other.buffer[sy][sx];
                }
            }
        }
    }

    pub fn horizontal_line(&mut self, y: usize, color: ColorRgb) {
        for x in 0..self.width {
            self.buffer[y][x] = color;
        }
    }

    pub fn vertical_line(&mut self, x: usize, color: ColorRgb) {
        for y in 0..self.height {
            self.buffer[y][x] = color;
        }
    }
}
