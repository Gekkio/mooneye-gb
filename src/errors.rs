use std::io;
use url;

error_chain! {
  types {
    MooneyeError, MooneyeErrorKind, MooneyeResultExt, MooneyeResult;
  }
  foreign_links {
    Io(io::Error);
    UrlParse(url::ParseError);
  }
  errors {
    BootromChecksum(crc32: u32) {
      description("Unrecognized boot ROM checksum")
      display("Unrecognized boot ROM checksum: 0x{:08x}", crc32)
    }
  }
}
