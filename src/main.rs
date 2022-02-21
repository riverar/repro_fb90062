#[test]
fn test_colors_features() {
    let reader = windows_metadata::TypeReader::get();

    // .class public auto ansi sealed import windowsruntime Microsoft.UI.Colors
    // extends [mscorlib]System.Object
    // ...
    // .method public hidebysig specialname static 
    // valuetype [System.Runtime.WindowsRuntime]Windows.UI.Color get_AliceBlue () runtime managed internalcall 
    //                                          ^^^^^^^^^^^^^^^^

    let typ = reader.expect_type_def(("Microsoft.UI", "Colors")).cfg();
    let features = typ.features("Microsoft");

    assert!(features.len() > 1);
    assert_eq!(features[0], "Microsoft.UI");
    assert_eq!(features[1], "Windows.UI");
}
