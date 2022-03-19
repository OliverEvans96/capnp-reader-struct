pub mod points_capnp {
    include!(concat!(env!("OUT_DIR"), "/proto/points_capnp.rs"));
}

use anyhow::Result;
use capnp::{message::ReaderOptions, serialize::SliceSegments};
use points_capnp::point;

pub struct ReaderWrapper<'a> {
    reader: capnp::message::Reader<SliceSegments<'a>>,
}

impl<'a> ReaderWrapper<'a> {
    fn print_point(&self) -> Result<()> {
        let point_reader = self.reader.get_root::<point::Reader>()?;
        println!("x = {:.2}", point_reader.get_x());
        println!("y = {:.2}", point_reader.get_y());

        Ok(())
    }
}

impl<'a> TryFrom<&'a [u8]> for ReaderWrapper<'a> {
    type Error = anyhow::Error;

    fn try_from(mut buffer: &'a [u8]) -> Result<ReaderWrapper<'a>> {
        let reader =
            capnp::serialize::read_message_from_flat_slice(&mut buffer, ReaderOptions::new())?;

        Ok(ReaderWrapper { reader })
    }
}

fn build_point_message(x: f32, y: f32) -> Result<Vec<u8>> {
    let mut msg_builder = capnp::message::Builder::new_default();
    let mut root_builder = msg_builder.get_root::<point::Builder>()?;
    root_builder.set_x(x);
    root_builder.set_y(y);
    let mut buf = Vec::new();
    capnp::serialize::write_message(&mut buf, &msg_builder)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let msg = build_point_message(13.6, 19.2)?;
    let wrapper: ReaderWrapper<'_> = msg.as_slice().try_into()?;
    wrapper.print_point()?;

    Ok(())
}
