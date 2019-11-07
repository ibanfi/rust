#!/bin/bash

# Set prompt
echo "export PS1=\"\[\e[1;91m\]\h:\W $\[\e[0m\] \"" >> .bash_profile

# Set LANG environments
echo "LANG=en_US.utf-8" > /etc/environment
echo "LC_ALL=en_US.utf-8" >> /etc/environment

# Install yum-config-manager
yum install -y yum-utils

# Add docker-ce repository
yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
# Add yarn repository
curl -sL https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo
# Add nodejs repository
curl -sL https://rpm.nodesource.com/setup_10.x | bash

# Install packages
yum install -y gcc-c++ make libtool net-tools git device-mapper-persistent-data lvm2 yarn nodejs docker-ce docker-ce-cli openssl-devel unzip postgresql-devel epel-release redis

# Start docker
systemctl enable docker
systemctl start docker

# Add vagrant user to docker group
usermod -g docker vagrant

# Install docker-compose
curl -L "https://github.com/docker/compose/releases/download/1.24.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# create rust target directory
# Workaround for shared folder linking issues
mkdir -p /opt/projects/target
chown -R vagrant:vagrant /opt/projects
