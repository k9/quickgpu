pub fn prettify(code: &[String]) -> String {
    let code = code.join("\n");
    let file = syn::parse_file(&code).unwrap();
    prettyplease::unparse(&file)
}
