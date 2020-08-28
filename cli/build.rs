use jstime_core as jstime;

fn main() {
    jstime::init(None);

    let options = jstime::Options::default();
    let data = jstime::JSTime::create_snapshot(options);

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("snapshot_data.blob");

    std::fs::write(&dest_path, data).unwrap();
}
