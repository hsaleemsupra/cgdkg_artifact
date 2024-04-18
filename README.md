# Non-interactive VSS using Class Groups and Application to DKG

## Overview


This repository contains the implementation code for the paper "Non-interactive VSS using Class Groups and Application to DKG."

## Running Benchmarks

The simplest way to run the benchmarks to compare Class Group-based DKG with Groth's DKG([paper](https://eprint.iacr.org/2021/339.pdf "paper")) is using a docker container

1. Install docker engine using these instructions: [docker engine](https://docs.docker.com/engine/install/ "docker engine") 

2. Once the docker engine is installed and running, clone this repo and build the docker image as follows:

    ```
     cd cgdkg_artifact
     docker build -t cgdkg_app .
    ```
    
3. After building the docker image, run it using:

    ```
     docker run --rm cgdkg_app
    ```

4. Running the docker container should run benchmarks comparing the **dealer time** and **receiver time** for our class group-based dkg with Groth's dkg for **n = [50, 100, 150, 200]**, t=n/2.

**NOTE**

Running benchmarks in Docker can be affected by the Docker runtime environment. The results might not fully reflect the performance characteristics that would be observed running directly on the host due to the overhead and resource limitations imposed by Docker.

Alternately, running the benchmarks directly on the host machine requires manually installing dependencies including, rust, cmake, clang, openssl, gmp. Here is an example of running the benchmarks directly on a Ubuntu host.

1. Install rust using the instructions here: [rust](https://www.rust-lang.org/tools/install "rust").

2. Install dependencies as follows:

    ```
    apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libclang-dev \
    openssl \
    libgmp-dev \
    && rm -rf /var/lib/apt/lists/* 
    ```

3. Clone the repo and run:

    ```
    cd cgdkg_artifact
    cargo build --release
    cargo bench
    ```
