use std::ffi::OsString;

pub fn string_to_args(string: &str) -> Vec<OsString> {
    // TODO: add handling of whitespace characters in quotes and character escaping
    let mut args = vec![OsString::from("photauri")];
    for arg in string.split_whitespace() {
        args.push(arg.into());
    }
    args
}
