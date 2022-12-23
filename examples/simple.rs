use littlefs::{Config, Result, FlashOperations, Error, Buffers};
use bytes::BufMut;

static mut READ_BUF: [u8; 256] = [0u8; 256];
static mut PROG_BUF: [u8; 256] = [0u8; 256];
static mut LOOKAHEAD_BUF: [u8; 64] = [0u8; 64];

struct InMemoryFlash(Vec<[u8; 256]>);

impl FlashOperations for InMemoryFlash {
    fn read(&mut self, block: u32, offset: u32, mut buffer: &mut [u8]) -> Result<()> {
        let block: &[u8] = self.0.get(block as usize).ok_or(Error::InvalidParam)?;
        let end = offset as usize + buffer.len();
        buffer.put_slice(&block[(offset as usize)..end]);

        Ok(())
    }

    fn prog(&mut self, block: u32, offset: u32, buffer: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn erase(&mut self, block: u32) -> Result<()> {
        unimplemented!()
    }

    fn sync(&mut self) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let flash = InMemoryFlash(Vec::new());
    let bufs = Buffers {
        read_buffer: unsafe { &mut READ_BUF },
        prog_buffer: unsafe { &mut PROG_BUF },
        lookahead_buffer: unsafe { &mut LOOKAHEAD_BUF }
    };
    let cfg = Config::new(512, 256, 4_000, 32);

    let fs = cfg.mount(flash, bufs).unwrap();
}