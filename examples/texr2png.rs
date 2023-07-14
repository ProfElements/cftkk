use std::{
    env,
    fs::{self, write, File},
    io::BufWriter,
};

use cftkk::texr::{Format, TexrReader};
use gctex::librii::rii_decode;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <texr>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();

    let texr = TexrReader::new(data).unwrap();

    let mut dest_data = vec![
        0u8;
        (texr.header().width * texr.header().height * 4)
            .try_into()
            .unwrap_or(0)
    ];

    let gx_format = match texr.header().texr_format {
        Format::Rgba8 => 0x6,
        Format::Rgb5a3 => 0x5,
        Format::Cmpr => 0xE,
        Format::Rgb565 => 0x4,
        Format::I4 => 0x0,
        Format::I8 => 0x1,
        Format::Ci8Rgb565 | Format::Ci8Rgb5a3 => 0x9,
        Format::Ci4Rgb565 | Format::Ci4Rgb5a3 => 0x8,
    };

    let tlut_format = match texr.header().texr_format {
        Format::Ci8Rgb565 | Format::Ci4Rgb565 => 0x1,
        Format::Ci4Rgb5a3 | Format::Ci8Rgb5a3 => 0x2,
        _ => 0,
    };

    rii_decode(
        &mut dest_data,
        texr.image_data(),
        texr.header().width,
        texr.header().height,
        gx_format,
        texr.texture_lookup_data().unwrap_or(&[]),
        tlut_format,
    );

    let mut encoder = png::Encoder::new(
        BufWriter::new(File::create(format!("{}.png", &args[1])).unwrap()),
        texr.header().width,
        texr.header().height,
    );

    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&dest_data).unwrap();
}
