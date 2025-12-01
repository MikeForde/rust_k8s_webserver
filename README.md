# simple_rust_webserver

referenced https://github.com/actix/examples/blob/master/basics/hello-world/

# OpenShift

devcontainer (incl. Dockerfile) + Dockerfile (for OpenShift) adjusted such that it can be deployed with pre-compiled binary as an experiment in:
1. Reducing compile time on OS.
2. By-passing the need to communicate with crates.io - as an alternative to the vendor approach - for a certain organisational flavour of OpenShift

# Mac ARM64
Devcontainer "--platform=linux/amd64" option included so will run in QEMU mode on Mac so compile binaries match.
This seems slower than alternatives, i.e.,
1. Running Ubuntu in UTM (on Mac).
2. Fatest naturally is not using Mac Arm64 and running devcontainer on native Linux (and presumably PC but not tried as yet).
