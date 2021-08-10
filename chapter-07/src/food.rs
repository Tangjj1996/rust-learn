pub fn run() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("this is something trouble???? {:?} {:?}", home, loopback);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("there are something intersting happen:: {:?}", &self)
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
