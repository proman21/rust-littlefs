pub enum Error {
    Io           = -5,
    Corrupt      = -84,
    NoDirEntry   = -2,
    Exists       = -17,
    NotDir       = -20,
    IsDir        = -21,
    NotEmpty     = -39,
    BadFileNo    = -9,
    FileTooBig   = -27,
    InvalidParam = -22,
    NoSpaceLeft  = -28,
    OutOfMem     = -12,
    NoAttr       = -61,
    NameTooLong  = -36
}

pub type Result<T> = core::result::Result<T, Error>;

pub enum MetadataType {
    Regular       = 0x001,
    Directory     = 0x002,
    // Internal types
    Splice        = 0x400,
    UserAttr      = 0x300,
    From          = 0x100,
    Globals       = 0x700,
    Checksum      = 0x500,
    // Internal type specializations
    Create        = 0x401,
    Delete        = 0x4ff,
    Superblock    = 0x0ff,
    DirStruct     = 0x200,
    CtzStruct     = 0x202,
    InlineStruct  = 0x201,
    SoftTail      = 0x600,
    HardTail      = 0x601,
    MoveState     = 0x7ff,
    // Internal chip sources
    FromNoop      = 0x000,
    FromMove      = 0x101,
    FromUserAttrs = 0x102
}