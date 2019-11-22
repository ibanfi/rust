Vagrant.configure("2") do |config|
    config.vm.box = "centos/7"
    config.vm.hostname = "rust-vm"
    config.vm.define "rust-vm"
  
    config.vm.synced_folder "~/.ssh/forte", "/home/vagrant/.ssh/forte"
    config.vm.network "forwarded_port", guest: 3010, host: 3010
    config.vm.network "forwarded_port", guest: 7770, host: 7770
    config.vm.network "forwarded_port", guest: 8770, host: 8770
    config.vm.network "forwarded_port", guest: 9770, host: 9770
    config.vm.network "forwarded_port", guest: 8088, host: 8088
    config.vm.network "forwarded_port", guest: 8090, host: 8090
    config.vm.network "forwarded_port", guest: 6379, host: 6379
    config.vm.network "forwarded_port", guest: 26257, host: 26257
    config.vm.network "forwarded_port", guest: 8081, host: 8081
  
     config.vm.provider "virtualbox" do |vb|
    # OSX workaround - disable microphone access
       vb.customize ["modifyvm", :id, "--audio", "none"]
       vb.memory = 4096
     end

     config.vm.provision "copy-files", type: "file", source: "res", destination: "$HOME/res"

     config.vm.provision "bootstrap", type: "shell", path: "res/bootstrap.sh"
    
     config.vm.provision "rust", type: "shell", path: "res/install-rust.sh", privileged: false

end
