## Introduction 
It is a remote control for neovim.
Press key from one device and it gets transferred to the neovim instance on another device within same wifi.

## Installation
Clone the repo and run
```sh
cargo build --release
cargo install --path .
```

## Usage
- Run neovim instance on another machine with this:
```nvim --listen ipaddr:6666 ```
- get ipaddr using your device settings, it is recommended to use static instad of DHCP ip 
- Connect to the instance using tsunagu
```tsunagu ipaddr:6666 ```
- and you will be presented with a grey screen, press any key and it gets redirected to neovim instance.

##### To exit, press '₹' key.
