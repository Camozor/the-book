#[derive(PartialEq, Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[allow(dead_code)]
#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

#[allow(dead_code)]
pub fn execute_chapter6() {
    route();
}

#[allow(dead_code)]
fn route() {
    let ip1 = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    let ip2 = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", ip1);
    println!("{:?}", ip2);
}

#[allow(dead_code)]
fn execute_match() {

}
