pub struct Buffers<'a> {
    pub read_buffer: &'a mut [u8],
    pub prog_buffer: &'a mut [u8],
    pub lookahead_buffer: &'a mut [u8]
}