extern crate pnet;
use pnet::datalink;

pub fn get_interfaces() {
    for iface in datalink::interfaces() {
        for ip in iface.ips.iter() {
            if ip.is_ipv4() && iface.name != "lo" {
                print!("Interface:{} Inet:{}; ", iface.name, ip);
            }
        }
    }
    print!("\n");
}

pub fn get_ip_net(net: &str) {
    for iface in datalink::interfaces() {
        for ip in iface.ips.iter() {
            if ip.is_ipv4() && iface.name == net {
                println!("Interface:{} Inet:{}", iface.name, ip);
            }
        }
    }
}
