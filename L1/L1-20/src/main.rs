trait ClientInterface {
    fn execute(&self);
}

struct Client;

impl ClientInterface for Client {
    fn execute(&self) {
        println!("hello world from client");
    }
}

struct Service;

impl Service {
    fn do_something(&self) {
        println!("something from service");
    }
}

struct Adapter {
    adaptee: Service,
}

impl Adapter {
    fn new(adaptee: Service) -> Self {
        Self { adaptee }
    }
}

impl ClientInterface for Adapter {
    fn execute(&self) {
        self.adaptee.do_something();
    }
}

fn call_method(client: impl ClientInterface) {
    client.execute();
}

fn main() {
    let client = Client;
    let service = Service;
    let adapter = Adapter::new(service);

    call_method(adapter);
    call_method(client);
    // call_method(service);    Service didn't implement ClientInterface
}