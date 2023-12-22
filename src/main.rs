use std::io;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use polling::{Event, Poller};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666")?;
    let local_addr = listener.local_addr()?;
    /*
    let local_addr = match listener.local_addr() {
        Result::Ok(l) => l,
        Result::Err(_) => "No local addr",
    };
    */
    println!("Listening on {local_addr}");

    let (stream, addr) = listener.accept()?;
    println!("Connection from {addr}");

    //Ok(())
    data_loop(stream)
}

fn data_loop(mut stream: TcpStream) -> std::io::Result<()> {
    let poller = Poller::new()?;
    let key_stream = 1;
    let key_stdin = 2;
    poller.add(&stream, Event::readable(key_stream))?;
    poller.add(&io::stdin(), Event::readable(key_stdin))?;

    let mut events = Vec::new();
    loop {
        let mut line = String::new();
        let mut rdata: [u8; 1500] = [0; 1500];
        //let mut rdata = String::with_capacity(1500);

        events.clear();
        poller.wait(&mut events, None)?;

        for ev in &events {
            if ev.key == key_stream {
                let rsize = match stream.read(&mut rdata) {
                    Result::Ok(0) => { println!("stream closed") ; return Ok(()) },
                    Result::Ok(rsize) => rsize,
                    Result::Err(e) => { println!("stream read error {e}") ; continue }
                };
                println!("Read {rsize} bytes");
                io::stdout().write(&rdata[0..rsize])?;
                poller.modify(&stream, Event::readable(ev.key))?;
            } else if ev.key == key_stdin {
                println!("Gimme a line:");
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");

                //line = line.trim().to_string();
                line = line.to_string();
                if line.ends_with("DONE\n") {
                    return Ok(());
                }

                match stream.write(line.as_bytes()) {
                    Result::Ok(_) => (),
                    Result::Err(e) => { println!("stream write error {e}") ; return Result::Err(e) }
                };
                poller.modify(&io::stdin(), Event::readable(ev.key))?;
            }
        }
    }

    //println!("{line}");
    //Ok(())
}
