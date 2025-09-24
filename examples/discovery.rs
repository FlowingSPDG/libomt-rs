fn main() {
    // Optionally set logging path or settings via env before calling into OMT.
    // std::env::set_var("OMT_STORAGE_PATH", "C:/ProgramData/OMT");

    let addrs = libomt_rs::discovery_getaddresses();
    if addrs.is_empty() {
        println!("No OMT sources discovered.");
        return;
    }
    println!("Discovered sources ({}):", addrs.len());
    for a in addrs {
        println!("{}", a);
    }
}



