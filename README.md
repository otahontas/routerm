# Routerm

Routerm is a command-line interface for getting walking/cycling/driving directions between points A and B. It uses [Openrouteservice](openrouteservice.org) API and is built with Rust.

## Installation and usage
- clone this repo
- [get API key for Openrouteservice](https://openrouteservice.org/dev/#/home)
- add you api key as env variable `ROUTESERVICE_API_KEY`
  - setting variable with `.env` -file is supported
- install binary with `cargo build --release`, then run `cd target/release`
- use routeplanner by running `cli-routeplanner start end`, where start and end are some addresses.
- alternatively skip binary installation and just run `cargo run start end` from the projects root

## Contributing
- Make a PR with new feature / bugfix.
- Check that your code style is inline with clippy, we check that with Github actions.

## Credits
- [@Halmela](https://github.com/Halmela)
- [@otahontas](https://github.com/otahontas)

## License
- MIT

## Project goals:
- possibility to select vehicle (foot, bicycle, car)
- set default vehicle
- set default addresses (home, work etc)
- TOML config file -based configuration
[x] give etas and vague directions
- Use API that isn't restricted with api keys
- Show map to user either with picture, ascii or link
- forward directions to favourite messages platform (telegram, irc,slack etc)
