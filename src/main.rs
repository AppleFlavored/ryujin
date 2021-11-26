use xcb::x;

fn main() -> xcb::Result<()> {
    // TODO: load configuration

    let (conn, screen_num) = xcb::Connection::connect(None)?;
    let setup = conn.get_setup();
    let screen = setup
        .roots()
        .nth(screen_num as usize)
        .unwrap();

    let values = [x::Cw::EventMask(
        x::EventMask::SUBSTRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_REDIRECT)];

    // T
    let cookie = conn.send_request_checked(
        &x::ChangeWindowAttributes {
            window: screen.root(),
            value_list: &values
        });
    conn.check_request(cookie)?;

    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(e) => match e {
                // TODO: Handle X events
                _ => {},
            }
            _ => {},
        }
    }
}