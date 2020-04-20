Vagrant.configure("2") do |config|
    config.vm.box = "centos/7"
    config.vm.hostname = "forte-vm"
    config.vm.define "forte-vm"

  
    config.vm.synced_folder "~/.ssh/forte", "/home/vagrant/.ssh/forte"
    config.vm.synced_folder "~/.aws/forte", "/home/vagrant/.aws"
    config.vm.network "forwarded_port", guest: 22, host: 2322, id: "ssh"
    config.vm.provider "virtualbox" do |vb|
    # OSX workaround - disable microphone access
       vb.customize ["modifyvm", :id, "--audio", "none"]
    # Minikube needs 2 CPUs
       vb.customize ["modifyvm", :id, "--cpus", 2]
       vb.memory = 4096
     end

     config.vm.provision "bootstrap", type: "shell", path: "res/bootstrap.sh"
    
     config.vm.provision "rust", type: "shell", path: "res/install-rust.sh", privileged: false

     config.vm.provision "java", type: "shell", path: "res/install-java.sh"
end
