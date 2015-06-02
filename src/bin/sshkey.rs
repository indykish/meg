extern crate megam_api;
extern crate term_painter;
extern crate meg;
//extern crate core;


use toml;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader,BufRead};
use std::str;
use std::path::Path;


use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;



use turbo;
use turbo::util::{CliResult, Config, TurboResult};
use self::megam_api::util::sshkeys::SSHKey;
//use self::megam_api::util::sshkeys::Success;

//use meg::util::parse_toml::Config;



#[derive(RustcDecodable)]
struct Options;

pub const USAGE: &'static str = "
Usage:
    meg sshkey [options]


Options:
    -h, --help              Print this message
    --list                  List SSHKeys
";


pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-sshkey; args={:?}", env::args().collect::<Vec<_>>());

//read from file
        //let data = ReadFile();


    /*    let opts = SSHKey {
           name:   format!("{}", "Megam"),
           accounts_id: format!("{}", "Systems"),
           path: format!("{}", "00"),
        }; 
        let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
           if x == "--create" {

            //let out = opts.create();

        match out {
        Ok(v) => {
            println!("success");
            println!("{}",
            Green.bold().paint("Hurray!! SSHKey is created! "));
        }
        Err(e) => {
            println!("Error parsing header");
          }
        }} else if x == "--list" {
            println!("{}",
            Green.bold().paint("Listing SSHKey"));
           let mut file = File::open("/home/yeshwanth/megam.toml");
          // let mut we = Config;
            //   we.load(file);
         /* let mut data = Vec::new();
           file.read_to_end(&mut data);
            let content = str::from_utf8(&data as &[u8]);
            match content {
                 Ok(v) =>  println!("{:?}", v),
                 Err(err) => println!("Error-------------->"),
            };
            let parser = toml::Parser::new(content); */
            println!("works!! ");


    }
}

return Ok(None)

*/
}
