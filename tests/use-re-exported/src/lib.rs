#[test]
fn test_use_re_exported_macro() {
    let file = file!();
    re_export_test::print!("print");
    re_export_test::eprint!("eprint");
    re_export_test::cprint!("cprint");
    re_export_test::ceprint!("ceprint");
    re_export_test::println!("println");
    re_export_test::eprintln!("eprintln");
    re_export_test::cprintln!("cprintln");
    re_export_test::ceprintln!("ceprintln");
    let (_, line1) = (re_export_test::dbg!("dbg"), line!());
    let (_, line2) = (re_export_test::edbg!("edbg"), line!());
    let (_, line3) = (re_export_test::cdbg!("cdbg"), line!());
    assert!(re_export_test::try_print!("try_print").is_ok());
    assert!(re_export_test::try_eprint!("try_eprint").is_ok());
    assert!(re_export_test::try_println!("try_println").is_ok());
    assert!(re_export_test::try_eprintln!("try_eprintln").is_ok());
    let (result, line4) = (re_export_test::try_dbg!("try_dbg"), line!());
    assert!(result.is_ok());
    let (result, line5) = (re_export_test::try_edbg!("try_edbg"), line!());
    assert!(result.is_ok());

    let chunks = re_export_test::take_chunks();
    let chunks: Vec<_> = chunks
        .iter()
        .map(|value| value.as_ref().map(String::as_str))
        .collect();
    assert_eq!(
        chunks,
        [
            Some("print"),
            Some("eprint"),
            Some("cprint"),
            Some("ceprint"),
            Some("println\n"),
            Some("eprintln\n"),
            Some("cprintln\n"),
            Some("ceprintln\n"),
            Some(&format!("[{file}:{line1}] \"dbg\" = \"dbg\"\n")),
            Some(&format!("[{file}:{line2}] \"edbg\" = \"edbg\"\n")),
            Some(&format!("[{file}:{line3}] \"cdbg\" = \"cdbg\"\n")),
            Some("try_print"),
            Some("try_eprint"),
            Some("try_println\n"),
            Some("try_eprintln\n"),
            Some(&format!("[{file}:{line4}] \"try_dbg\" = \"try_dbg\"\n")),
            Some(&format!("[{file}:{line5}] \"try_edbg\" = \"try_edbg\"\n"))
        ]
    );
}
