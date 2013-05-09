extern mod linenoise;

mod rush {
    extern mod linenoise;
    extern mod core;
    use core::run;

    pub fn exec_command(stdin_fd: libc::c_int, command: &str) -> libc::c_int {
        let pipe_out = os::pipe();
        let pipe_err = os::pipe();
        let mut args: ~[~str] = ~[];
        let mut prog = ~"";
        let mut first = true;
        str::each_split_char(command, ' ', |item| {
            if (first) {
                prog = copy item.to_owned();
                first = false;
            } else {
                args.push(item.to_owned());
            }
            true
        });
        let pid = run::spawn_process(
                prog, args, &None, &None,
                stdin_fd, pipe_out.out, pipe_err.out);
        os::close(pipe_out.out);
        os::close(pipe_err.out);
        do task::spawn_sched(task::SingleThreaded) || {
            run::waitpid(pid);
        }
        pipe_out.in
    }

    pub fn handle_input(line: &str) {
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        let mut commands: ~[&str] = ~[];
        str::each_split_char_no_trailing(line, '|', |command| { commands.push(str::trim(command)); true });
        let pipe_in = os::pipe();
        let std_out = vec::foldl(pipe_in.in, commands, |stdin, command| {
            exec_command(stdin, *command)
        });
        os::close(pipe_in.out);
        do task::spawn_sched(task::SingleThreaded) || {
            io::print(core::run::readclose(std_out));
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
