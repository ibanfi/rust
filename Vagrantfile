Vagrant.configure("2") do |config|
    config.vm.box = "centos/7"
    config.vm.hostname = "rust-vm"
    config.vm.define "rust-vm"
  
    config.vm.synced_folder "projects", "/home/vagrant/projects"
    config.vm.network "forwarded_port", guest: 3010, host: 3010
    config.vm.network "forwarded_port", guest: 7770, host: 7770
    config.vm.network "forwarded_port", guest: 7780, host: 7780
  
     config.vm.provider "virtualbox" do |vb|
    # OSX workaround - disable microphone access
       vb.customize ["modifyvm", :id, "--audio", "none"]
       vb.memory = 2048
     end

     config.vm.provision "copy-files", type: "file", source: "res", destination: "$HOME/res"

     config.vm.provision "bootstrap", type: "shell", path: "res/bootstrap.sh"
    
     config.vm.provision "rust", type: "shell", path: "res/install-rust.sh", privileged: false

end
