use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use webgl::{TextureCompression,PixelFormat};


pub fn parse(dds: &[u8]) -> (u16,u16,Vec<u8>,TextureCompression,PixelFormat) {
    println!("dds size {}", dds.len() );
    let mut buf = Cursor::new(dds);
    // ;
    // 542327876
    let magic = buf.read_u32::<LittleEndian>()
        .expect("Error::IOError");
    if magic != 542327876 {
        panic!("Err(ErrorKind::InvalidDDSFile.into())");
    };
    buf.set_position(12);
    let width = buf.read_u32::<LittleEndian>()
        .expect("little endian read failed");
    let height = buf.read_u32::<LittleEndian>()
        .expect("little endian read failed");

    let mut offset = 0;
    
    #[cfg(target_arch = "wasm32")]
    {
        offset = 32*4; // dds header size
    };

    (
        width as _,
        height as _,
        dds[offset..].to_vec(),
        TextureCompression::RgbaDxt5,
        PixelFormat::Rgba,
    )
}