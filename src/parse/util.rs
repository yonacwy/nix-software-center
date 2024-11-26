pub fn checkonline() -> bool {
    reqwest::blocking::get("http://nmcheck.gnome.org/check_network_status.txt").is_ok()
}
