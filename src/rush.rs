extern mod linenoise;

mod rush {
    extern mod linenoise;
    extern mod core;
    use core::task::spawn;
    use core::run;

    pub fn exec_command(stdin_fd: libc::c_int, command: &~str) -> libc::c_int {
        let pipe_out = os::pipe();
        let pipe_err = os::pipe();
        let com = copy *command;
        do spawn{
            let command_split = str::split_char(com, ' ');
            let prog = command_split.head();
            let args = command_split.tail();
            let pid = run::spawn_process(
                    *prog, args, &None, &None,
                    stdin_fd, pipe_out.out, pipe_err.out);
            os::close(pipe_out.out);
            os::close(pipe_err.out);
            let status = run::waitpid(pid);
        }
        pipe_out.in
    }

    pub fn handle_input(line: &str) {
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        let commands = vec::map(str::split_char(line, '|'), |command| str::trim(*command));
        let pipe_in = os::pipe();
        let std_out = vec::foldl(pipe_in.in, commands, exec_command);
        os::close(pipe_in.out);
        let (stdout_po, stdout_ch) = comm::stream();
        do task::spawn_sched(task::SingleThreaded) || {
            stdout_ch.send(core::run::readclose(std_out));
        }
        let stdout = stdout_po.recv();
        io::print(stdout);
    }

}

fn main() {
    linenoise::set_multiline(true);
    loop {
        let line = linenoise::init("rush: ");
        rush::handle_input(line);
    }
}
