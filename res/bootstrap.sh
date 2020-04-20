#!/bin/bash

# Set prompt
echo "export PS1=\"\[\e[1;91m\]\h:\W $\[\e[0m\] \"" >> .bash_profile
echo "eval \`ssh-agent\`" >> .bash_profile
echo "ssh-add ~/.ssh/forte/id_rsa" >> .bash_profile

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

# Add git repo
yum install -y https://centos7.iuscommunity.org/ius-release.rpm
yum remove git*

# Install packages
yum install -y gcc-c++ make libtool net-tools git2u-all device-mapper-persistent-data lvm2 yarn nodejs docker-ce docker-ce-cli openssl-devel unzip postgresql-devel epel-release redis lsof golang gmp-devel python-pip jq

# Start docker
systemctl enable docker
systemctl start docker

# Add vagrant user to docker group
usermod -g docker vagrant

# Install docker-compose
curl -sL "https://github.com/docker/compose/releases/download/1.24.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Install AWS CLI
pip install awscli --upgrade --user

# Install helm 2
curl -sL https://get.helm.sh/helm-v2.16.3-linux-amd64.tar.gz | tar -zx
tar -zxf helm.tar.gz
mv linux-amd64/helm /usr/local/bin/helm
rm helm.tar.gz
rm -rf linux-amd64
