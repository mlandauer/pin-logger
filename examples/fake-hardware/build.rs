fn main() {
    pin_logger::build::scan_source_for_pin_logs(std::path::Path::new("pin_logs.txt"));
}
