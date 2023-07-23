pub(crate) mod scan_func;
pub(crate) mod ui;

fn do_selection(selection:u32){
    match selection {
        1 => {
            let (ip_address, min_port, max_port) = scan_func::ScanFunc::basic_scan_params();
            scan_func::ScanFunc::basic_scan(&ip_address, min_port, max_port);
        },
        2 => {
            let (ip_address, min_port, max_port) = scan_func::ScanFunc::basic_scan_params();
            scan_func::ScanFunc::basic_scan(&ip_address, min_port, max_port);
        },
        3 => {
            scan_func::ScanFunc::simple_scan_most_common_ports();
        },
        _ => {
            println!("Invalid selection");
        }
    }
}

fn main() {
    ui::UI::print_menu();
    let selection = ui::UI::get_input().parse::<u32>().expect("Invalid selection");
    do_selection(selection);

}
