extern mod linenoise;


fn handle_input(line: ~str) {
    linenoise::history_add(line);
    linenoise::history_save("history.txt");
    let split_line = str::split_char(line, ' ');
    let command = split_line.head();
    let args = split_line.tail();
    core::run::run_program(*command, args);
}

fn main() {
    linenoise::set_multiline(true);
    loop {
        let line = linenoise::init("rush: ");
        handle_input(line);
    }
}
