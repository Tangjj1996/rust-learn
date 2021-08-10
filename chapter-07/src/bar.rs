pub fn run() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    fn route(ip_kind: IpAddrKind) {
        println!("there are many kindof ip_kind: {:?}", ip_kind)
    }
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("there are many function {:?} {:?}", home, loopback);
}
