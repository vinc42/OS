use crate::io;

pub fn shell() {
    loop {
        io::print(b"[our_shell]$ ");

        loop {
            let c = io::getchar();
            if c == b'\r' || c == b'\n' {
                io::print(b"\n");
                break;
            }
            io::putchar(c);
        }

        // TODO: manage the user input
    }
}
