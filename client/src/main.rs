mod limit;

use crate::limit::{LimitReq, LimitType};
use livesplit_hotkey::{Hotkey, KeyCode, Modifiers};
use std::io::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.2.1:65432")?;

    let mut req_limit_data = LimitReq {
        state: false,
        limit: LimitType::Data,
    };

    let mut req_limit_instance = LimitReq {
        state: false,
        limit: LimitType::Instance,
    };


    let hook = livesplit_hotkey::Hook::new().unwrap();

    let mut stream1 = stream.try_clone().unwrap();

    hook.register(
        Hotkey {
            key_code: KeyCode::KeyX,
            modifiers: Modifiers::CONTROL,
        },
        move || {
            req_limit_data.state = !req_limit_data.state;
            stream1.write(&*bincode::serialize(&req_limit_data).unwrap()).unwrap();
        },
    ).unwrap();

    let mut stream2 = stream.try_clone().unwrap();

    hook.register(
        Hotkey {
            key_code: KeyCode::KeyZ,
            modifiers: Modifiers::CONTROL,
        },
        move || {
            req_limit_instance.state = !req_limit_instance.state;
            stream2.write(&*bincode::serialize(&req_limit_instance).unwrap()).unwrap();
        }
    ).unwrap();

    loop {}
}
