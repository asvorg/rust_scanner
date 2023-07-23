pub(crate) mod scan_func;

fn main() {
    let (ip_address, min_port, max_port) = scan_func::ScanFunc::basic_scan_params();
    scan_func::ScanFunc::basic_scan(&ip_address, min_port, max_port);

}
