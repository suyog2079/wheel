use std::{net::{TcpListener, TcpStream}, collections::HashMap, io::Error, fs::OpenOptions, };
use std::io::BufReader;
use std::io::prelude::*;



enum User {
    Bob,
    Sam,
    Joe,
    Ram
}


fn get_user_map() -> HashMap<String, User> {
    let mut map = HashMap::new();

    map.insert("BOB".to_owned(), User::Bob);
    map.insert("SAM".to_owned(), User::Sam);
    map.insert("JOE".to_owned(), User::Joe);
    map.insert("RAM".to_owned(), User::Ram);


    map

}


fn append_magnet_link(user : &User, magnet: &str) -> Option<()> {
    let file_to_open = match user {
	User::Bob => "bob.txt",
	User::Sam => "sam.txt",
	User::Joe => "joe.txt",
	User::Ram => "ram.txt"

    };


    let mut file_options = OpenOptions::new();

    file_options.append(true);

    let mut file = file_options.open(file_to_open).ok()?;
    
    file.write_all(magnet.as_bytes()).ok()?;

    file.write_all("\n".as_bytes()).ok()?;

    file.flush().ok()?;
    
    Some(())
}


fn handle_connection(mut stream: TcpStream) -> Option<()> {
    let map = get_user_map();


    let buf_reader = BufReader::new(&mut stream);


    let two_lines : Vec<String> = buf_reader
	.lines()
	.map(|l| l.unwrap())
	.take(2)
	.collect();


    println!("{:#?}", two_lines);

    let (_, user) = map.get_key_value(two_lines.get(0)?)?;
    let magnet = two_lines.get(1)?;

    append_magnet_link(user, magnet);
    
    Some(())
}

fn main() {

    // start a tcp socket


    let listener = TcpListener::bind("0.0.0.0:8080")
	.expect("ERROR: couldn't start tcp socket in port 8080");

    for stream in listener.incoming() {

	println!("CONNECTION ESTABLISHED");


	let _ = if let Ok(stream)  = stream {

	    let mut send_stream = stream.try_clone().unwrap();

	    match handle_connection(stream) {
		Some(_t) => send_stream.write("OK".as_bytes()),
		None =>  send_stream.write("BAD".as_bytes()),
	    }

	} else {
	    continue;
	};


    }
}
