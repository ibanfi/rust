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


## Run Rust apps

If you're ready to use the vagrant dev environment, login to it:
`vagrant ssh`

Further steps are the same on both (local/vm) environments
```
cd projects/actix-basic
cargo run
```