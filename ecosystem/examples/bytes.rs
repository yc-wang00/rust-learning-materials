use anyhow::Result;

use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
    // let bytes = Bytes::from_static(b"hello world");
    let mut buf = BytesMut::with_capacity(10);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);
    buf.put_i64(0xdeadbeef);

    let a = buf.split();
    let b = buf.freeze();
    println!("{:?}", a);
    println!("{:?}", b);

    Ok(())
}
