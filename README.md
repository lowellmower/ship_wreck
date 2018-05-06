### ship_wreck

A small, extremely simple binary used to monitor the health of docker containers

### Basic Usage
```
$ ship_wreck --help
Ship Wreck 0.0.1
Lowell M. <lowell.mower@gmail.com>
Health monitoring for Docker containers

USAGE:
    ship_wreck [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

$ ship_wreck -c test.yml
Client { connection: "/var/run/docker.sock" }
Connecting client...

HTTP/1.0 200 OK
Api-Version: 1.37
Content-Length: 796
Content-Type: application/json
Date: Sun, 06 May 2018 20:36:13 GMT
Docker-Experimental: true
Ostype: linux
Server: Docker/18.03.0-ce (linux)

[
   {
      "Id":"4c90a8966f432aa9c37269e4d3f51d9a67ea63a080cfaa72b5e7f803009df7ee",
      "Names":[
         "/lucid_shockley"
      ],
      "Image":"alpine:3.5",
      "ImageID":"sha256:074d602a59d7c317fb5d3056622e5361308aa72ebbfab895bbab7b317d9aef1b",
      "Command":"/bin/sh",
      "Created":1524505744,
      "Ports":[

      ],
      "Labels":{

      },
      "State":"running",
      "Status":"Up 39 minutes",
      "HostConfig":{
         "NetworkMode":"default"
      },
      "NetworkSettings":{
         "Networks":{
            "bridge":{
               "IPAMConfig":null,
               "Links":null,
               "Aliases":null,
               "NetworkID":"294c1730acb5743bb7792dab94b27a4640a0f78c527527494a275f0f641477be",
               "EndpointID":"d04c4864d4c6489a5122d78b31f26b4bd44a2f3e2ab61e5f9923fccebad0af08",
               "Gateway":"172.17.0.1",
               "IPAddress":"172.17.0.2",
               "IPPrefixLen":16,
               "IPv6Gateway":"",
               "GlobalIPv6Address":"",
               "GlobalIPv6PrefixLen":0,
               "MacAddress":"02:42:ac:11:00:02",
               "DriverOpts":null
            }
         }
      },
      "Mounts":[

      ]
   }
]
```
