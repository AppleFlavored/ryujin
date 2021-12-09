use wm::Ryujin;
use xcb::Connection;

mod wm;

fn main() -> xcb::Result<()> {
    let (conn, screen_num) = Connection::connect(None)?;
    let screen = conn
        .get_setup()
        .roots()
        .nth(screen_num as usize)
        .unwrap();

    let wm = Ryujin::new(&conn, screen);
    wm.run()
}