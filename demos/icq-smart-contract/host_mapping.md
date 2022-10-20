config.yaml host section configuration ip address maps according to below mapping. 

host:
1. rpc address 

config.yml
rpc: ":26659"

config.toml:
[rpc]
# TCP or UNIX socket address for the RPC server to listen on
laddr = "tcp://0.0.0.0:26659"

2.  p2p address 

config.yml
p2p: ":26661"

config.toml:
[p2p]

# Address to listen for incoming connections
laddr = "tcp://0.0.0.0:26661"

3. prof 
config.yml
prof: ":6061"

config.toml:
# pprof listen address (https://golang.org/pkg/net/http/pprof)
pprof_laddr = ":6061"

4. grpc 

config.yml
grpc: ":9095"

app.toml
[grpc]
  address = ":9095"
  enable = true

5. grpc-web

config.yml   
grpc-web: ":8091"

app.toml
[grpc-web]
  address = ":8091"

6. api 
   
config.yml
  api: ":1311"

app.toml
[api]
  address = "tcp://0.0.0.0:1311"

7. frontend
   
config.yml
frontend: ":8081"
dev-ui: ":12351"

no mapping in the config directory. These two are specific to frontend application.


### for this demo

field	|   default	| quasar	| cosmos hub	| osmosis				
rpc	    |   26657	| 26659	    | 26669	        | 26679
p2p	    |   26656	| 26661	    | 26663	        | 26662
prof	|   6060	| 6061	    | 6063	        | 6062
grpc	|   9090	| 9095	    | 9097	        | 9096
grpc-web|	9091	| 8081	    | 8093	        | 8092
api	    |   1313	| 1311	    | 1313	        | 1312
