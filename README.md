## Introduction 
It is a remote control for neovim.
Press key from one device and it gets transferred to the neovim instance on another device within same wifi.

## Installation
Clone the repo and run
```sh
cargo install --path main
```

## Usage
- Run neovim instance on another machine with this:
```nvim --listen ipaddr:6666 ```
- get ipaddr using your device settings, 
  it is recommended to use static instead of DHCP ip 
- Connect to the instance using tsunagu
```tsunagu ipaddr:6666 ```
- and you will be presented with a "Connected to {address}" message.
  Press any key and it gets redirected to neovim instance instantly.

##### To exit, press '₹' key.

## Code Structure
The code consists of main and utilcro directories,
 each being a rust sub project in themselves.
The utilcro contains utility macros, and main has the actual app.
