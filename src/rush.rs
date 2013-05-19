mod rush {
    extern mod core;
    extern mod linenoise;
    use core::run;

    pub struct Job {
        command: ~str,
        stdin: os::Pipe,
        stdout: os::Pipe,
        stderr: os::Pipe,
    }

    impl Job {
        pub fn exec(&self) -> i32 {
            let mut args: ~[~str] = ~[];
            let mut prog = ~"";
            let mut first = true;
            str::each_split_char(self.command, ' ', |item| {
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
                    self.stdin.in, self.stdout.out, self.stderr.out);
            os::close(self.stdin.in);
            if (self.stdout.out != libc::STDOUT_FILENO as libc::c_int) {
                os::close(self.stdout.out);
                os::close(self.stderr.out);
            }
            pid
        }
    }

    pub fn handle_input(line: &str) {
        if (line.is_empty()) { return; }
        let mut commands: ~[~str] = ~[];
        str::each_split_char_no_trailing(line, '|', |command| {
            commands.push(command.to_owned());
            true
        });
        let mut pipeline: ~[Job] = ~[];
        let mut index = 0;
        let mut stdin = os::Pipe{ in: -1, out: -1};
        commands.each(|command| {
            index += 1;
            // Needs to be initialized like this?
            // Surely I'm missing a nicer syntax for this?
            let mut stdout = os::Pipe{ in: -1, out: -1};
            let mut stderr = os::Pipe{ in: -1, out: -1};
            if (index==commands.len()) {
                // Pass the last job the stdout/stderr for the shell itself
                stdout = os::Pipe{
                    out: libc::STDOUT_FILENO as libc::c_int,
                    in: -1,
                };
                stderr = os::Pipe{
                    out: libc::STDERR_FILENO as libc::c_int,
                    in: -1,
                };
            } else {
                stdout = os::pipe();
                stderr = os::pipe();
            }
            pipeline.push(Job{
                command: str::trim(*command).to_owned(),
                stdin: stdin,
                stdout: stdout,
                stderr: stderr,
            });
            stdin = stdout;
            true
        });

        let mut pid = -1;
        pipeline.each(|job| { pid = job.exec(); true });
        os::waitpid(pid);
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
