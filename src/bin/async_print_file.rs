use std::env::args;

use async_std::{fs::File, io, prelude::*, task};
const LEN: usize = 16 * 1024; // 16Kb

fn main() -> io::Result<()> {
    let path = args().nth(1).expect("missing path argument");
    task::block_on(async {
        let mut file = File::open(&path).await?;
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; LEN];
        loop {
            let n = dbg!(file.read(&mut buf).await?);
            if n == 0 {
                stdout.flush().await?;
                return Ok(());
            }
            stdout.write_all(&buf[..n]).await?;
        }
    })
}
