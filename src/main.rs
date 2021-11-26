use xcb::{Event, x};

fn main() -> xcb::Result<()> {
    // TODO: load configuration

    let (conn, screen_num) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();
    let screen = setup
        .roots()
        .nth(screen_num as usize)
        .unwrap();

    let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
        window: screen.root(),
        value_list: &[
            x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_REDIRECT)
        ]
    });

    if let Err(_) = conn.check_request(cookie) {
        println!("Another window manager is already running.");
        std::process::exit(1);
    }

    loop {
        match conn.wait_for_event()? {
            Event::X(e) => match e {
                x::Event::MapRequest(e) => {
                    println!("{:?}", e.window());
                }
                _ => {},
            }
            _ => {},
        }
    }
}