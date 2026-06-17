# Chat with websocket in Rust 💬

This is a toy project written to learn Rust and websocket.

I used Gemini to generate some functions and the HTML code but the goal was to 
understand how to write Rust code and how websockets work.

## 📝 Features

Get the web interface on http://localhost:3000/.

You will be asked to enter a pseudo, then you will see the page with all the available chat rooms. 
After choosing a room or creating a new one, you will be able to send messages in it, 
and see previous messages.

For now, all data (users, messages, rooms) are stored in memory (no database).

⚠️ The authentication is not secured, no password is required. This is a toy project.

### Routes

- GET / : retrieve the html content
- GET /login : enter pseudo and retrieve token
- GET /chat/{room_id} : begin websocket for given room
- GET /chat/{room_id}/history : retrieve given room history
- GET /rooms : list all rooms
- POST /rooms : create a new room
- DELETE /rooms/{room_id} : delete a room

### 🛠️ Improvements (TODO)

- Add a redis connection to handle data storage (rooms, users, messages) hence the application could be scaled horizontally
- Add a database for persistant storage
- Add proper user authentication
- Handle already read messages
- Handle "typing users"
- Write unit tests

## 🚀 Build and Run

### Local version

```bash
cargo run
```

### Docker version

The docker version is very light thanks to multi-layer build and the "scratch" docker image.

```bash
docker build -t chat:scratch .
docker run --rm -p 3000:3000 --init chat:scratch
```
