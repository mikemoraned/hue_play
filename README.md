# Playing with Hue API via Rust

- Use https://developers.meethue.com/develop/get-started-2/ to register locally. You could also use https://github.com/kali/hue.rs/blob/main/src/bin/hue_register_user.rs but I found the other instructions first.
- stick in a `.env` file with the format `USERNAME=....`
- use program via `cargo run` and it will dump all lights and then toggle one light on or off
