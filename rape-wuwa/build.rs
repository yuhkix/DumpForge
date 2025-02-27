use embed_resource::compile;

fn main() {
    if cfg!(target_os = "windows") {
        compile("resources.rc", embed_resource::NONE).manifest_optional().unwrap();
    }
}