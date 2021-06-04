// communicator 모듈은 모든 크레이트의 최상위 모듈이므로, root module이라고 부름
extern crate communicator; 

fn main() {
    communicator::client::connect();
}