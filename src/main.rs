extern crate hueclient;
use hueclient::bridge::Bridge;
fn main() {
    let username = dotenv::var("USERNAME").unwrap();
    println!("Loaded username");
    let bridge = Bridge::discover().unwrap().with_user(username);
    println!("Hue bridge found: {:?}", bridge);

    match bridge.get_all_lights() {
        Ok(lights) => {
            println!("id name                 on    bri   hue sat temp  x      y");
            for ref l in lights.iter() {
                println!(
                    "{:2} {:20} {:5} {:3} {:5} {:3} {:4}K {:4} {:4}",
                    l.id,
                    l.light.name,
                    if l.light.state.on { "on" } else { "off" },
                    if l.light.state.bri.is_some() {
                        l.light.state.bri.unwrap()
                    } else {
                        0
                    },
                    if l.light.state.hue.is_some() {
                        l.light.state.hue.unwrap()
                    } else {
                        0
                    },
                    if l.light.state.sat.is_some() {
                        l.light.state.sat.unwrap()
                    } else {
                        0
                    },
                    if l.light.state.ct.is_some() {
                        l.light
                            .state
                            .ct
                            .map(|k| if k != 0 { 1000000u32 / (k as u32) } else { 0 })
                            .unwrap()
                    } else {
                        0
                    },
                    if l.light.state.xy.is_some() {
                        l.light.state.xy.unwrap().0
                    } else {
                        0.0
                    },
                    if l.light.state.xy.is_some() {
                        l.light.state.xy.unwrap().1
                    } else {
                        0.0
                    },
                );
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            ::std::process::exit(2)
        }
    }
}
