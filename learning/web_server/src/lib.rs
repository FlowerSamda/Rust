use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {

    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 뮤텍스 얻고, 언랩 후, recv를 호출해 Job을 얻음
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} god a job; executing.", id);

                // (*job)()  이게 이제 FnBox의 역할로 넘어감 (이미 박스 밖으로 나온 상태임!)
                job.call_box();
            }
        });
        
        Worker{
            id,
            thread,
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
    sender: mpsc::Sender<Job>,
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
        self.sender.send(job).unwrap();
    }


}


