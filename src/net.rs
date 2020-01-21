extern crate pnet;
use pnet::datalink;

pub fn get_interfaces() {
    for iface in datalink::interfaces() {
        println!("{:?}", iface.ips);
    }
}
