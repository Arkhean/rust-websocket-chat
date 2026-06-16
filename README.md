# Chat with websocket in Rust

This is a toy project written to learn Rust and websocket.

I used Gemini to generate some functions and the HTML code but the goal was to 
understand how to write Rust code and how websockets work.

## Features

TODO...

## Build and Run

```bash
cargo run
```

## Docker version

The docker version is very light thanks to multi-layer build and the "scratch" docker image.

```bash
docker build -t chat:scratch .
docker run --rm -p 3000:3000 --init chat:scratch
```
