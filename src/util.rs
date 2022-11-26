use regex::Regex; // 1.1.8

struct NameParts {
    num: i32,
    short_name: String,
    icon: String
}

fn parse_workspace_name(name: String) {
    let seperator = Regex::new(r"(?P<num>\d+):?(?P<shortname>\w+)? ?(?P<icons>.+)?").expect("Invalid regex");
    let splits = split_keep(&seperator, name);
    return NameParts(splits[0], splits[1], splits[2]);
}

fn construct_workspace_name(parts: NameParts) {
    let new_name = parts.num;
    if parts.num || parts.icon {
        new_name = new_name + ":";

        if (parts.short_name) {
            new_name = new_name + parts.short_name;
        }

        if (parts.icons) {
            new_name = new_name + parts.icons;
        }
    }
    return new_name;
}

fn icon_for_window(window: i32i32) {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let wm_class = get_class(conn, window)
}

fn get_class(conn: &Connection, id: &i32) -> String {
    let window: xproto::Window = *id as u32;
    let long_length: u32 = 8;
    let mut long_offset: u32 = 0;
    let mut buf = Vec::new();
    loop {
        let cookie = xproto::get_property(
            &conn,
            false,
            window,
            xproto::ATOM_WM_CLASS,
            xproto::ATOM_STRING,
            long_offset,
            long_length,
        );
        match cookie.get_reply() {
            Ok(reply) => {
                let value: &[u8] = reply.value();
                buf.extend_from_slice(value);
                match reply.bytes_after() {
                    0 => break,
                    _ => {
                        let len = reply.value_len();
                        long_offset += len / 4;
                    }   
                }
            }   
            Err(err) => {
                println!("{:?}", err);
                break;
            }   
        }
    }
    let result = String::from_utf8(buf).unwrap();
    let results: Vec<&str> = result.split('\0').collect();
    results[0].to_string()
}