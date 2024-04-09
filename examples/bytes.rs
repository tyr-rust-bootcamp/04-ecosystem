use anyhow::Result;
use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello world\n");
    buf.put(&b"goodbye world"[..]);
    buf.put_i64(0xdeadbeef);

    println!("{:?}", buf);
    let a = buf.split();
    let mut b = a.freeze(); // inner data cannot be changed

    let c = b.split_to(12);
    println!("{:?}", c);

    println!("{:?}", b);
    println!("{:?}", buf);

    Ok(())
}
