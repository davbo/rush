#[link_args="-ledit -ltermcap"]
use libc::{c_char};
extern mod edit {
    fn el_init(argv: **c_char, stdin: io::Reader, stdout: io::Writer, stderr: io::Writer) -> *u8;
    fn el_set(el: *u8, param: libc::c_int, val: ~str);
}
pub const EL_EDITOR: libc::c_int = 2_i32;
fn main() {
    let args = os::args().map(|arg| str::as_c_str(*arg, |buf| buf));
    unsafe {
        let el = edit::el_init(vec::raw::to_ptr(args), io::stdin(), io::stdout(), io::stderr());
        let editor = ~"vi";
        edit::el_set(el, EL_EDITOR, editor);
    }
}
