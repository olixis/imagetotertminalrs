extern crate image;

use image::GenericImageView;

use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn main() {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open("megaman.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    let mut bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    let mut cont = 0;

    for pixel in img.pixels(){
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(pixel.2[0],pixel.2[1],pixel.2[2]))));
        if cont % img.dimensions().0 == 0 {
            writeln!(&mut buffer, "\u{2588}");
        } else{
            write!(&mut buffer, "\u{2588}");
        }
        //write!(&mut buffer, ".");
        bufwtr.print(&buffer);
        buffer.clear();
        cont+=1;
    }

    buffer.reset();
    bufwtr.print(&buffer);



    // Write the contents of this image to the Writer in PNG format.
    //img.save("test.png").unwrap();
}
