# VM with predefined Rust

## Prerequisites

1. Install vagrant: https://www.vagrantup.com/downloads.html
2. Clone the repository: `git clone https://github.com/ibanfi/vm-env-rust.git`
3. Install the vagrant-vbguest plugin: `vagrant install vagrant-vbguest`

## Build environment

```
cd vm-env-rust
vagrant up
```

## Use the environment (connect to vm)

`vagrant ssh`

## Stop/start environment

`vagrant halt/vagrant up`

## Remove entire environment

`vagrant destroy`

