fn main () {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "org_keienb_file_dialog_demo_async.gresource"
    );
}