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

## Run Rust apps

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
### Create a user
Request #1

`curl -X POST http://localhost:3020/api/v1/users -H "Content-Type: application/json" -d  '{ "name": "Martin Fowler", "email": "martin@martinfowler.com", "id": 11 }' -v`

Request #2

`curl -X POST http://localhost:3020/api/v1/users -H "Content-Type: application/json" -d  ' { "name": "MF", "email": "bad", "id": 11 }' -v`