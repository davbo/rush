extern mod linenoise;

mod rush {
    extern mod linenoise;
    extern mod core;
    use core::task::spawn;
    use core::comm::{stream,Port,Chan,SharedChan};

    fn write_str(wr: io::Writer, s: &str) { str::byte_slice(s, |v| wr.write(v)) }

    fn exec_command(command: &~str) -> Port<~str> {
        let (strout,strin): (Port<~str>, Chan<~str>) = stream();
        let com = copy *command;
        do spawn {
            let command_split = str::split_char(str::trim(com), ' ');
            let prog = command_split.head();
            let args = command_split.tail();
            let mut prog = core::run::start_program(*prog, args);
            let rd = prog.output();
            let buf = io::with_bytes_writer(|wr| {
                let mut bytes = [0, ..4096];
                while !rd.eof() {
                    let nread = rd.read(bytes, bytes.len());
                    wr.write(bytes.view(0, nread));
                }
            });
            strin.send(str::from_bytes(buf));
        }
        strout
    }

    pub fn handle_input(line: &str) {
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        let commands = str::split_char(line, '|');
        let mut stdouts : ~[Port<~str>] = core::vec::map(commands, exec_command);
        for stdouts.each |stdout| {
            io::println(stdout.recv());
        }
    }

}

fn main() {
    linenoise::set_multiline(true);
    loop {
        let line = linenoise::init("rush: ");
        rush::handle_input(line);
    }
}
