# Command 1 - Download OpenJDK
curl -O https://download.java.net/java/GA/jdk13.0.1/cec27d702aa74d5a8630c65ae61e4305/9/GPL/openjdk-13.0.1_linux-x64_bin.tar.gz

# Command 2 -- Extract downloaded file
tar xvf openjdk-13.0.1_linux-x64_bin.tar.gz

# Command 3 -- Move extracted folder to /opt
sudo mv jdk-13.0.1 /opt/

# Command 4 -- Configure Java environment
cat <<EOF | sudo tee /etc/profile.d/jdk13.sh
export JAVA_HOME=/opt/jdk-13.0.1
export PATH=\$PATH:\$JAVA_HOME/bin
EOF

# Command 5 -- Source Profile
source /etc/profile.d/jdk13.sh

