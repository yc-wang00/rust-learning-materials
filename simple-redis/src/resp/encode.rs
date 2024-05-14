use crate::{RespEncode, SimpleString};

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {

}

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!(":{}\r\n", self.0).into_bytes()
    }
}