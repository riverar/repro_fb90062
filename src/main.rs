#[test]
fn test_colors_features() {
    let reader = windows_metadata::TypeReader::get();

    // .class public auto ansi sealed import windowsruntime Microsoft.UI.Colors
    // extends [mscorlib]System.Object
    // ...
    // .method public hidebysig specialname static 
    // valuetype [System.Runtime.WindowsRuntime]Windows.UI.Color get_AliceBlue () runtime managed internalcall 
    //                                          ^^^^^^^^^^^^^^^^

    let mut features = std::collections::HashSet::new();
    reader.expect_type_def(("Microsoft.UI", "Colors"))
        .methods()
        .flat_map(|m| m.cfg().features("Microsoft"))
        .for_each(|m| { let _ = &features.insert(m); } );

    println!("Returned unique features\n{:#?}", features);

    assert!(features.len() > 1);
    assert_eq!(features.contains("Microsoft.UI"), true);
    assert_eq!(features.contains("Windows.UI"), true);
}
