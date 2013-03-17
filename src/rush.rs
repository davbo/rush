extern mod linenoise;

mod rush {
    extern mod linenoise;
    extern mod core;
    use core::task::spawn;
    use core::comm::{stream,Port,Chan,SharedChan};

    struct CommandIO {
        in: Chan<~str>,
        out: Port<~str>
    }

    fn write_str(wr: io::Writer, s: &str) {
        io::println("Writing str");
        io::println(s);
        str::byte_slice(s, |v| wr.write(v))
    }

    fn exec_command(command: &~str) -> CommandIO {
        let (stdout_output, stdout_input): (Port<~str>, Chan<~str>) = stream();
        let (stdin_output, stdin_input): (Port<~str>, Chan<~str>) = stream();
        let com = copy *command;

        do spawn {
            let command_split = str::split_char(str::trim(com), ' ');
            let prog = command_split.head();
            let args = command_split.tail();
            let mut prog = core::run::start_program(*prog, args);
            if stdin_output.peek() {
                write_str(prog.input(), stdin_output.recv());
            }
            let rd = prog.output();
            let buf = io::with_bytes_writer(|wr| {
                let mut bytes = [0, ..4096];
                while !rd.eof() {
                    let nread = rd.read(bytes, bytes.len());
                    wr.write(bytes.view(0, nread));
                }
            });
            stdout_input.send(str::from_bytes(buf));
        }
        CommandIO {in: stdin_input, out: stdout_output}
    }

    fn link_channels(prog1: &CommandIO, prog2: &CommandIO) {
        io::println("link1");
        if prog1.out.peek() {
            prog2.in.send(prog1.out.recv());
        }
        io::println("link2");
    }

    pub fn handle_input(line: &str) {
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        let commands = str::split_char(line, '|');
        let (strout, strin): (Port<~str>, Chan<~str>) = stream();
        let mut initial_io = CommandIO {in: strin, out: strout};
        let io_channels = vec::map(commands, exec_command);
        let last = io_channels.last();
        loop {
            io_channels.each(|&io| {
                link_channels(&initial_io, &io);
                initial_io = io;
                true
            });
            if last.out.peek() {
                io::println(last.out.recv());
            }
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
