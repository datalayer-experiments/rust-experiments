use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
extern crate ctrlc;
extern crate exitcode;

#[pyfunction]
fn spawn_websocket_server(){

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();  
    println!("{:?}", server);

    ctrlc::set_handler(move || {
        println!("Abort");
        std::process::exit(exitcode::OK);
    })
    .expect("Error setting Ctrl-C handler");
    


    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
    
                println!("{:?}",msg);
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}




#[pymodule]
fn websocket_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spawn_websocket_server, m)?)?;

    Ok(())
}