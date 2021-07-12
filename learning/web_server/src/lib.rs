use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Message를 받고, 이에 따라서 행동함
                // 뮤텍스 얻고, 언랩 후, recv를 호출해 Message를 얻음
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        
                        // (*job)()  이게 이제 FnBox의 역할로 넘어감 (이미 박스 밖으로 나온 상태임!)
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });
        
        Worker{
            id,
            thread: Some(thread),
        }
    }
}

// Box를 사용하기 위한 트릭
trait FnBox {
    fn call_box(self: Box<Self>);
}

// FnOnce() 트레잇을 구현하는 F 타입에 대해 FnBox트레잇을 구현
// -> FnOnce() 클로저가 FnBox의 call_box 메소드를 사용할 수 있다는 뜻
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()  // 클로저를 Box<T> 밖으로 빼내고 호출
    }
}

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");


        // 반복문을 두번으로 나눈 이유
        // -> 두개 worker가 있을 시, 반복문을 한번만 호출한다면, 첫번쨰 반복자에서 종료메시지가
        // 채널로 전송되고, 첫 worker의 스레드에서 join 호출
        // 그런데, 첫번째 worker가 요청을 처리하느라 바쁘면 두번째 worker가 채널에서 종료메시지를 가져와 종료
        // 첫번쨰는 그럼 영원히 종료 안됨

        // 그래서, terminate를 모든 workers에게 보내고나서, join을 호출하게되어 join 호출 전에
        // 모두가 종료 메시지를 수신하게 될 것이라고 확신 가능!

        for worker in &mut self.workers {
            println!("Shuting down worekr {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                // worker의 스레드가 이미 None일 시, worker가 자신의 스레드를 이미 정리했다는 뜻으로 아무 일도 하지 않음
                thread.join().unwrap();  // Option의 take는 Some(var)에서 var를 빼내 None으로 대체

            }
        }
    }
}


impl ThreadPool {
    /// 새 스레드풀 생성
    ///
    /// size는 풀 안의 스레드 개수입니다.
    /// 
    /// #panics
    /// 
    /// 'new' 함수는 size가 0일때 패닉을 일으킵니다.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  //  스레드 개수가 1개 이상이어야 하므로
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // Vec::with_capacity(size)는 벡터의 공간을 미리 할당하여 new보다 효율이 높음
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn spawn<F, T>() {

    }

    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}


enum Message{
    NewJob(Job),  // 스레드가 실행되어야 할 job을 담고있는 NewJob 변수
    Terminate,    // 스레드 중지 
}