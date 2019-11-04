# Rust

## Prerequisites

1. Install vagrant: https://www.vagrantup.com/downloads.html
2. Clone the repository: `git clone https://github.com/ibanfi/rust.git`

## Build environment

If you don't want to use the vm environment, go to _Run Rust apps_ section

```
cd rust
vagrant up
```
## Use the environment (connect to vm)

`vagrant ssh`

## Stop/start environment

`vagrant halt/vagrant up`

## Remove entire environment

`vagrant destroy`

If you're ready to use the vagrant dev environment, login to it:

`vagrant ssh`

## Run Rust app

Further steps are the same on both (local/vm) environments
```
cd projects/actix-basic
cargo run
```

## Test it using cargo test
Open a new terminal and run
```cargo test```

## Test it using curl
Open a new terminal and these commands:
### Get users
Request

`curl http://localhost:3020/api/v1/users`

Response
```
[{"id":1,"name":"Leanne Graham","email":"Sincere@april.biz"},{"id":2,"name":"Ervin Howell","email":"Shanna@melissa.tv"},{"id":3,"name":"Clementine Bauch","email":"Nathan@yesenia.net"},{"id":4,"name":"Patricia Lebsack","email":"Julianne.OConner@kory.org"},{"id":5,"name":"Chelsey Dietrich","email":"Lucio_Hettinger@annie.ca"},{"id":6,"name":"Mrs. Dennis Schulist","email":"Karley_Dach@jasper.info"},{"id":7,"name":"Kurtis Weissnat","email":"Telly.Hoeger@billy.biz"},{"id":8,"name":"Nicholas Runolfsdottir V","email":"Sherwood@rosamond.me"},{"id":9,"name":"Glenna Reichert","email":"Chaim_McDermott@dana.io"},{"id":10,"name":"Clementina DuBuque","email":"Rey.Padberg@karina.biz"}]
```
### Create a user
Request #1

`curl -X POST http://localhost:3020/api/v1/users -H "Content-Type: application/json" -d  '{ "name": "Martin Fowler", "email": "martin@martinfowler.com", "id": 11 }' -v`

Response
```
* About to connect() to localhost port 3020 (#0)
*   Trying ::1...
* Connection refused
*   Trying 127.0.0.1...
* Connected to localhost (127.0.0.1) port 3020 (#0)
> POST /api/v1/users HTTP/1.1
> User-Agent: curl/7.29.0
> Host: localhost:3020
> Accept: */*
> Content-Type: application/json
> Content-Length: 73
>
* upload completely sent off: 73 out of 73 bytes
< ** HTTP/1.1 200 OK **
< content-length: 92
< date: Mon, 04 Nov 2019 13:26:11 GMT
<
Successful upload: User { id: 11, name: "Martin Fowler", email: "martin@martinfowler.com" }
* Connection #0 to host localhost left intact```

Request #2

`curl -X POST http://localhost:3020/api/v1/users -H "Content-Type: application/json" -d  ' { "name": "MF", "email": "bad", "id": 11 }' -v`

Response
```
* About to connect() to localhost port 3020 (#0)
*   Trying ::1...
* Connection refused
*   Trying 127.0.0.1...
* Connected to localhost (127.0.0.1) port 3020 (#0)
> POST /api/v1/users HTTP/1.1
> User-Agent: curl/7.29.0
> Host: localhost:3020
> Accept: */*
> Content-Type: application/json
> Content-Length: 43
>
* upload completely sent off: 43 out of 43 bytes
< ** HTTP/1.1 422 Unprocessable Entity **
< content-length: 17
< date: Mon, 04 Nov 2019 13:27:02 GMT
<
Validation error
* Connection #0 to host localhost left intact
```