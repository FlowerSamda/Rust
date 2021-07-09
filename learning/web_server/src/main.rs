use std::io::prelude::*;    // 스트림으로부터 읽고 쓰는 것을 허용하는 특성에 접근할 수 있도록 함 
use std::net::TcpListener;  
use std::net::TcpStream;
use std::fs::File;

fn main() {
    // TCP 연결 수신을 할 IP주소  * 80포트는 관리자 권한 필요, 비 관리자는 1024이상 포트만 사용 가능
    let listener = TcpListener::bind("127.0.0.1:8545").unwrap();

    // incoming 메소드는 스트림의 차례에 대한 반복자를 반환(tcpStream)
    // strean은 클라이언트-서버 간의 열려있는 커넥션을 의미 
    // * connection: 클라이언트와 서버간의 연결-응답생성-연결을 끊는 요청과 응답 과정 전반을 의미
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}



// mut stream을 선언한 이유: TcpStream 인스턴스가 내부에서 어떤 데이터가 반환되는지 추적하기 때문!
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];  // 버퍼를 읽을 데이터를 저장할 스택에 선언 (512바이트)
    stream.read(&mut buffer).unwrap();  // TcpStream으로부터 읽어들인 바이트를 버퍼로 집어넣음

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}",
    status_line,
    contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();  // flush: 모든 바이트들이 커넥션으로 쓰여질 때까지 프로그램을 대기
}


/*
Method: Request: GET
REQUEST_URI: /
HTTP-version: HTTP/1.1

headers: Host: 이후는 모두 헤더
message-body:

Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
Connection: keep-alive
sec-ch-ua: " Not;A Brand";v="99", "Google Chrome";v="91", "Chromium";v="91"
sec-ch-ua-mobile: ?0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,**;q=0.8,application/signed-exchange;v=b3;q=0.9
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
*/