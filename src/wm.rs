use xcb::{ Connection, x::{self, Screen} };

pub struct Ryujin<'a> {
    conn: &'a Connection,
    screen: &'a Screen,
}

impl<'a> Ryujin<'a> {
    pub fn new(conn: &'a Connection, screen: &'a Screen) -> Ryujin<'a> {
        Ryujin { conn, screen }
    }

    pub fn run(&self) -> xcb::Result<()> {
        let cookie = self.conn.send_request_checked(&xcb::x::ChangeWindowAttributes {
            window: self.screen.root(),
            value_list: &[x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT | x::EventMask::SUBSTRUCTURE_NOTIFY
            )]
        });
    
        if let Err(_) = self.conn.check_request(cookie) {
            println!("Another window manager is already running.");
            std::process::exit(1);
        };

        loop {
            match self.conn.wait_for_event()? {
                _ => {},
            }
        }
    }
}