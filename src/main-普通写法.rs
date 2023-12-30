use std::io;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};

// UDP消息
const MESSAGE: &[u8] = b"\x01\x00\x00\x00\xcb\x9c\xf0\xe7\x3b\x5c\xaa\xb2\x5b\x14\x62\x35\x57\x90\x3c\xa1\x55\xa3\xb1\x55\x44\x66\x3f\x17\xc7\xf3\x93\x9e\x6e\xfa\x95\x8d\xc9\xf2\x84\x57\x23\x88\xb6\x93\x1c\x0b\x74\xbb\x11\x98\x37\x61\x2b\x54\xeb\xb9\x4e\x24\x5b\x90\xf7\xd0\x4c\xe8\xcb\x50\xec\xda\x61\xa7\x3b\xc2\x77\xe6\x58\x76\x12\xaf\x2c\x0e\x29\x0b\x01\x31\x6f\x75\x1f\x67\x3f\x33\x2b\x0b\xa5\x6e\x53\xf3\x34\x82\x59\xec\xf7\x3c\xcb\x6e\x03\x1b\x6d\xa3\x12\x90\x34\xa3\x4f\x89\x1f\x20\x1c\x3e\x7f\xe3\xd7\x21\x9f\xdc\x2f\x4d\xb0\xff\x53\x13\xb3\x0f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

fn udp(address: &str, port: u16) -> Result<(), String> {
    let addr: SocketAddr = format!("{}:{}", address, port)
        .parse()
        .map_err(|_| "无效地址或端口".to_string())?;

    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;

    // 发送数据
    socket.send_to(MESSAGE, &addr).map_err(|e| e.to_string())?;

    // 设置超时时间
    socket
        .set_read_timeout(Some(std::time::Duration::new(2, 0)))
        .map_err(|e| e.to_string())?;

    // 接收回复
    let mut buffer = [0; 100];
    let (_size, _) = socket.recv_from(&mut buffer).map_err(|e| e.to_string())?;
    
    //let response = &buffer[..size];
    //println!("Received: {:?}", response);

    Ok(())
}

fn wait_for_enter() {
    print!("按Enter键退出程序...");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

fn main() {
    loop {
        let mut input = String::new();
        print!("输入要查询的IP地址：");
        io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let ip = input.trim();

        // 检查输入的IP地址是否有效
        if let Ok(ip_address) = ip.parse::<std::net::IpAddr>() {
            let ports = vec![
                854, 859, 864, 878, 880, 890, 891, 894, 903, 908, 928, 934, 939, 942, 943, 945,
                946, 955, 968, 987, 988, 1002, 1010, 1014, 1018, 1070, 1074, 1180, 1387, 1843,
                2371, 2506, 3138, 3476, 3581, 3854, 4177, 4198, 4233, 5279, 5956, 7103, 7152, 7156,
                7281, 7559, 8319, 8742, 8854, 8886, 2408, 500, 4500, 1701,
            ];
            for port in ports {
                match udp(&ip_address.to_string(), port) {
                    Ok(_) => println!("{} {} ——> OK", ip, port),
                    Err(err) => println!("{} {} ——> Error: {}", ip, port, err),
                }
            }
            break;
        } else {
            println!("IP 地址格式无效。请重试: ");
        }
    }
    wait_for_enter();
}
