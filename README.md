# rosmaster-discovery
Small tool for discovering the rosmaster URI on a local network using dns-sd

## Usage
Make sure avahi is running on your system to broadcast services (obtained on Ubuntu with apt package `avahi-daemon`) and copy `rosmaster.service` into `/etc/avahi/services/`. You can verify it's running with `avahi-browse -a` which should have an entry like below:

```
+ enp3s0 IPv4 rosmaster on <hostname>                       _ros._tcp            local
```

You can store it in an environment variable like so:
```
export ROS_MASTER_URI=$(rosmaster-discovery)
```
