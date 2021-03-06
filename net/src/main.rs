use std::net::{IpAddr, Ipv4Addr, SocketAddr,SocketAddrV6,Ipv6Addr,TcpStream,TcpListener,Shutdown};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    //net_ip_addr();
    //net_tcp();
    //net_ip();
    //net_tcp_client();
    net_tcp_server();
}

fn net_tcp_client(){
    //=============客户端读写================
    let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    stream.read(&mut[0; 64]);
    stream.write(&[1]);
    stream.flush();
    //stream.shutdown(Shutdown::Write).expect("同时关闭写连接失败");
    //stream.shutdown(Shutdown::Read).expect("同时关闭读连接失败");
    stream.shutdown(Shutdown::Both).expect("同时关闭读写连接失败");

}
fn net_tcp_server(){
     fn handle_client(stream: TcpStream) {
       let bulider=  thread::Builder::new().name("foo".into()).stack_size(10);
       let handler=  bulider.spawn(move||{
           let tcp=stream.try_clone().expect("clone TcpStream fail");
           tcp.set_read_timeout(Some(Duration::from_secs(1)));
           tcp.set_write_timeout(Some(Duration::from_secs(1)));

         }).expect("任务执行失败！！！");
         handler.join().unwrap();
     }
     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
     for stream in listener.incoming() {
         handle_client(stream.unwrap());
     }
}

fn net_ip(){
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    assert_eq!("127.0.0.1".parse(), Ok(localhost));
    assert_eq!(localhost.is_loopback(), true);
}


fn net_tcp(){
    let stream = TcpStream::connect(("127.0.0.1", 443));
    let stream = TcpStream::connect("127.0.0.1:443");
    let stream = TcpStream::connect((Ipv4Addr::new(127, 0, 0, 1), 443));

}

//这个方法注释部分有问题  抽空处理一下
fn net_ip_addr() {

    let socket =SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)),8080);
    assert_eq!("0.0.0.0:8080".parse(), Ok(socket));
    assert_eq!(socket.port(),8080);
    assert_eq!(socket.ip(),IpAddr::V4(Ipv4Addr::new(0,0,0,0)));
    assert_ne!(socket.is_ipv4(),false);
    //=============================================
    let socket = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
     assert_eq!("[2001:db8::1]:8080".parse(), Ok(socket));
     assert_eq!(socket.ip(), &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
     assert_eq!(socket.port(), 8080);
    //=======================================================
    let addr = SocketAddr::from(([127, 0, 0, 1], 443));
    //let addrs_iter = addr.to_socket_addrs();
    //=======================================
    //let addrs_iter = "localhost:443".to_socket_addrs().unwrap();
    //==================================================

}