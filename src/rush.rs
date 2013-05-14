mod rush {
    extern mod core;
    extern mod linenoise;
    use core::run;

    pub struct JobOutput {
        stdout: libc::c_int,
        stderr: libc::c_int,
        pid: libc::c_int,
    }

    pub fn exec_command(input: JobOutput, command: &str) -> JobOutput {
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
                input.stdout, pipe_out.out, pipe_err.out);
        os::close(input.stdout);
        os::close(pipe_out.out);
        os::close(pipe_err.out);
        JobOutput{
            pid: pid,
            stdout: pipe_out.in,
            stderr: pipe_err.in,
        }
    }

    pub fn handle_input(line: &str) {
        let mut commands: ~[&str] = ~[];
        str::each_split_char_no_trailing(line, '|', |command| { commands.push(str::trim(command)); true });
        let fake_out = JobOutput{
            stdout: -1,
            stderr: -1,
            pid: -1,
        };
        let job_output = vec::foldl(fake_out, commands, |stdin, command| {
            exec_command(stdin, *command)
        });
        if (job_output.pid == -1) { return }
        proxy_fds(job_output.stdout, libc::STDOUT_FILENO as libc::c_int);
        proxy_fds(job_output.stderr, libc::STDERR_FILENO as libc::c_int);
        run::waitpid(job_output.pid);
    }
    pub fn proxy_fds(input: libc::c_int, output: libc::c_int) {
        do task::spawn_sched(task::SingleThreaded) || {
            unsafe {
                let output = io::fd_writer(output, false);
                let file = os::fdopen(input);
                let reader = io::FILE_reader(file, false);
                let mut bytes = [0, ..4096];
                while !reader.eof() {
                    let nread = reader.read(bytes, bytes.len());
                    output.write(bytes.slice(0, nread));
                }
                os::fclose(file);
            }
        }
    }
    pub fn main() {
        linenoise::set_multiline(true);
        loop {
            prompt();
        }
    }
    pub fn prompt() {
        let line = linenoise::init("rush: ");
        linenoise::history_add(line);
        linenoise::history_save("history.txt");
        handle_input(line);
    }
}
