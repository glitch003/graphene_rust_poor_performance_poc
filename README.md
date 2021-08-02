# Graphene proof of concept with Rust

## Background
I am seeing some weird issues with Rust + Graphene that I think are related to locking or race conditions of some kind, where the Rust process ends up hanging forever.

The repo is designed to be run using the .sh scripts, which will spawn 3 graphene-sgx processes which each run the binary "poc". This is designed to mimic a distributed system with 3 "nodes"

Inside src/main.rs is a HTTP server that has 2 endpoints. The endpoint at "/" simply returns the string "Hello, World". The endpoint at "/test" will hit the "/" endpoint of all 3 servers (including itself) 3 times.

Running curl.sh will call the "/test" endpoint on all the servers.  When run without graphene, this works fine and completes instantly. Running in graphene-sgx, we see mixed behavior. Sometimes it all works fine. Sometimes it works but takes 5 seconds to run (it should be nearly instantaneous). Sometimes it works on 2 of the 3 servers and the 3rd one hangs forever.  Sometimes all 3 servers hang forever.

I am running the 1.2-rc1 release of Graphene on Ubuntu 20.04.  

Things I tried that did not work: 
* changing the "rpc_thread_num" variable to match "thread_num" to enable exitless mode.  
* running strace on the graphene-sgx process (maybe I didn't understand how to use the output to figure out the problem)
* checked htop to see if the CPU is doing work when the process hangs.  It is not.
* setting "insecure__allow_eventfd" to false, which stops the app from running at all.

## Prereqs
You may need to install the "concurrently" binary via npm with "npm i -g concurrently" to use the launch scripts.  You can also avoid this by manually running the following commands instead of using the launch scripts:

```
sudo ROCKET_PORT=7470 graphene-sgx poc &
sudo ROCKET_PORT=7471 graphene-sgx poc &
sudo ROCKET_PORT=7472 graphene-sgx poc &
```

## Running
First, run "./build.sh" which will compile and sign the enclave.

Then, run "./launch_with_graphene.sh" to run with graphene, or, "launch.sh" to run without graphene.

Finally, run "./curl.sh" which will hit the "/test" endpoint of all the servers. 

If it works fine, run it again.  Usually after the 2nd or 3rd run it will hang forever.

Note that if you run it without graphene, it completes instantly, every time.

## Goal
Diagnose the problem and solve it, so that the program runs the same way inside graphene as it does without graphene