extern crate hueclient;
use std::env;

#[allow(dead_code)]
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return
    }
    let bridge = ::hueclient::bridge::Bridge::discover_required().with_user(args[1].to_string());
    match bridge.get_all_lights() {
        Ok(lights) => {
            for ref l in lights.iter() {
                println!("{:2} {:20} {:5} {:3} {:5} {:3}", l.id, l.light.name,
                    if l.light.state.on {"on"} else {"off"} ,
                    l.light.state.bri, l.light.state.hue, l.light.state.sat);
            }
        },
        Err(err) => panic!(err)
    }
}
