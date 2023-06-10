use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

use once_cell::sync::Lazy;

static CHUNKS: Lazy<Mutex<Vec<Option<String>>>> = Lazy::new(Mutex::default);

pub fn take_chunks() -> Vec<Option<String>> {
    use core::mem::take;
    take(&mut CHUNKS.lock().unwrap())
}

pub fn write_fn(value: String) {
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(Some(value));
}

pub fn flush_fn() {
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(None);
}

custom_print::define_macros!(#[macro_export] {
    print,
    eprint,
    cprint,
    ceprint,
    println,
    eprintln,
    cprintln,
    ceprintln,
    dbg,
    edbg,
    cdbg,
    try_print,
    try_eprint,
    try_println,
    try_eprintln,
    try_dbg,
    try_edbg,
}, concat, $crate::write_fn);

custom_print::define_macros!(#[macro_export] {
    flush,
    eflush,
    try_flush,
    try_eflush,
}, concat, $crate::flush_fn);

#[test]
fn test_re_export_macro() {
    let file = file!();
    cprint!("cprint");
    ceprint!("ceprint");
    cprintln!("cprintln");
    ceprintln!("ceprintln");
    let (_, line1) = (edbg!("edbg"), line!());
    let (_, line2) = (cdbg!("cdbg"), line!());
    assert!(try_print!("try_print").is_ok());
    assert!(try_eprint!("try_eprint").is_ok());
    assert!(try_println!("try_println").is_ok());
    assert!(try_eprintln!("try_eprintln").is_ok());
    let (result, line3) = (try_dbg!("try_dbg"), line!());
    assert!(result.is_ok());
    let (result, line4) = (try_edbg!("try_edbg"), line!());
    assert!(result.is_ok());

    let chunks = take_chunks();
    let chunks: Vec<_> = chunks
        .iter()
        .map(|value| value.as_ref().map(String::as_str))
        .collect();
    assert_eq!(
        chunks,
        [
            Some("cprint"),
            Some("ceprint"),
            Some("cprintln\n"),
            Some("ceprintln\n"),
            Some(&format!("[{file}:{line1}] \"edbg\" = \"edbg\"\n")),
            Some(&format!("[{file}:{line2}] \"cdbg\" = \"cdbg\"\n")),
            Some("try_print"),
            Some("try_eprint"),
            Some("try_println\n"),
            Some("try_eprintln\n"),
            Some(&format!("[{file}:{line3}] \"try_dbg\" = \"try_dbg\"\n")),
            Some(&format!("[{file}:{line4}] \"try_edbg\" = \"try_edbg\"\n"))
        ]
    );
}
