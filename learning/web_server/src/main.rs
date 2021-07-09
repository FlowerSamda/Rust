use std::net::TcpListener;

fn main() {
    // TCP 연결 수신을 할 IP주소  * 80포트는 관리자 권한 필요, 비 관리자는 1024이상 포트만 사용 가능
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming 메소드는 스트림의 차례에 대한 반복자를 반환(tcpStream)
    // strean은 클라이언트-서버 간의 열려있는 커넥션을 의미 
    // * connection: 클라이언트와 서버간의 연결-응답생성-연결을 끊는 요청과 응답 과정 전반을 의미
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }

}