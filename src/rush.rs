extern mod linenoise;



fn run_command(command: &~str) -> core::os::Pipe {
    io::println(*command);
    let pipe = core::os::pipe();
    let command_split = str::split_char(*command, ' ');
    let command_name = command_split.head();
    let command_args = command_split.tail();
    core::run::spawn_process(*command_name, command_args, ~None, ~None, pipe.in, pipe.out, pipe.out);
    pipe
}

fn handle_input(line: &str) {
    linenoise::history_add(line);
    linenoise::history_save("history.txt");
    let mut pipes = ~[];
    let commands = str::split_char(line, '|');
    for commands.each |command| {
        pipes.push(run_command(command));
    }
}

fn main() {
    linenoise::set_multiline(true);
    loop {
        let line = linenoise::init("rush: ");
        handle_input(line);
    }
}
