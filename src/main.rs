#![allow(non_snake_case)]
#![allow(dead_code)]
//extern crate nom;
extern crate reqwest;
extern crate threadpool;
extern crate clap;

use clap::{Arg, App};  // get cmdline arguments
use threadpool::ThreadPool; // makes this go WAY faster
use std::io::BufRead; // Read File line by line
//use reqwest::blocking::Client; // Connect / Set get requests to web servers
use std::fs::File; // open files
use std::error::Error; // For boxing all errors
use std::path::Path; // Check if file exists.



enum Proxy { // proxy or no proxy enum
	With{Addr: String},
	Without
}
enum SSL { // accept or not accept invalid certificates.
	Secure,
	Insecure
}
enum BasicAuth {
	With{User: String, Pass: String},
	Without
}



fn build_client(secure: SSL, proxy: Proxy, auth: BasicAuth) -> Result<reqwest::blocking::Client, Box<dyn Error>> {
	match secure {
		SSL::Secure => {
			match proxy {
				Proxy::With {Addr} => {
					let prox = format!("{}", Addr);
					match auth {
						BasicAuth::Without => {
						 	Ok(reqwest::blocking::Client::builder()
							    .proxy(reqwest::Proxy::all(&prox)?)
							    .timeout(::std::time::Duration::from_secs(5))
							    .build()?
							)
						}
						BasicAuth::With {User, Pass} => {
							Ok(reqwest::blocking::Client::builder()	  
								.proxy(reqwest::Proxy::all(&prox)?.basic_auth(&User, &Pass))
								.timeout(::std::time::Duration::from_secs(5))
								.build()?
								)
						}
					}
				}
				Proxy::Without => {
					Ok(reqwest::blocking::Client::builder()
						.timeout(::std::time::Duration::from_secs(5))
						.build()?
					)
				}
			}
		}
		SSL::Insecure => {
			match proxy {
				Proxy::With {Addr} => {
					let prox = format!("{}", Addr);
					match auth {
						BasicAuth::Without => {
						 	Ok(reqwest::blocking::Client::builder()
							    .proxy(reqwest::Proxy::all(&prox)?)
							    .timeout(::std::time::Duration::from_secs(5))
							    .build()?
							)
						}
						BasicAuth::With {User, Pass} => {
							Ok(reqwest::blocking::Client::builder()
								.proxy(reqwest::Proxy::all(&prox)?.basic_auth(&User, &Pass))
								.timeout(::std::time::Duration::from_secs(5))
								.build()?
								)
						}
					}
				}
				Proxy::Without => {
					Ok(reqwest::blocking::Client::builder()
						.timeout(::std::time::Duration::from_secs(5))
						.danger_accept_invalid_certs(true)
						.build()?
					)
				}
			}
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = App::new("UrlTester-rs")
		.version("0.1.2")
		.author("Daniel Hoberecht")
		.about("Test Web Proxies from list of addresses")
		.arg(Arg::with_name("file")
			.help("File of urls to use")
			.short("f")
			.long("file")
			.takes_value(true)
			)
		.arg(Arg::with_name("Proxy")
			.help("Proxy")
			.short("p")
			.long("proxy")
			.takes_value(true)
			)
		.arg(Arg::with_name("Threads")
			.help("number of thread to use")
			.short("t")
			.long("threads")
			.takes_value(true)
			)
		.arg(Arg::with_name("User")
			.help("Basic Proxy Auth Username")
			.short("u")
			.long("user")
			.takes_value(true)
			)
		.arg(Arg::with_name("Pass")
			.help("Basic Proxy Auth Password")
			.short("P")
			.long("pass")
			.takes_value(true)
			)
		.get_matches();
		let file = matches.value_of("file").unwrap_or("urls.lst");
		let _proxy = matches.value_of("Proxy");
		let threads = matches.value_of("Threads");
		let user_auth = matches.value_of("User");
		let pass_auth = matches.value_of("Pass");
	/*
	let ip = String::from("127.0.0.1"); // can be cloned to allow Proxy access accross multiple threads
	let port = String::from("8080"); // can be cloned to allow Proxy access accross multiple threads
	*/ 
	let pool = match threads { 
		None => {ThreadPool::new(10)} // create ThreadPool with 10 threads
		_ 	 => {ThreadPool::new(threads.unwrap().parse::<usize>()?)}
	};
	if Path::new(&file).exists() { // check if file exists or unwrap to urls.lst or quit
		let file_r = File::open(file)?; // open file
		let reader = ::std::io::BufReader::new(file_r); // create buffered reader of file
		for (_, line) in reader.lines().enumerate() { // iterate over file with buffered
			let ssl = SSL::Insecure; // Set ssl to enum SSL::Insecure
			//let proxy = Proxy::With {Ip: ip.clone(), Port: port.clone()}; 
			let proxy = match _proxy {
				None => {Proxy::Without},
				_ => {Proxy::With{Addr: _proxy.unwrap().to_string()}}
			};
			let auth = match user_auth {
				None => {BasicAuth::Without},
				_ =>  {
					match pass_auth {
						None => {BasicAuth::Without},
						_ => {
							BasicAuth::With {
								User: user_auth.unwrap_or(&String::from("Anonymous")).to_string(), 
								Pass: pass_auth.unwrap_or(&String::new()).to_string()
							}
						}
					}
				}
			};
			let url = format!("https://{}", line?);
			pool.execute(move || {
				match build_client(ssl,proxy,auth).unwrap().get(&url).send() {
					Ok(c)  => {println!("{} {}",&url, c.status());},
					Err(e) => {eprintln!("{}", e.to_string());}	
				}
			});
		}
	} else {
		println!("{:?} does not exist",&file);
		::std::process::exit(0);
	}
	pool.join();
	if pool.panic_count() > 0 {println!("{:?}", pool.panic_count());}
	
	Ok(())
}

