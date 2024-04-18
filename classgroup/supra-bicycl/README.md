## supra-bicycl

This repository provides rust bindings for the [Bicycl library (C++)](https://github.com/Entropy-Foundation/BICYCL/tree/dev) implementating various class group operations. You do not need to install the Bicycl C++ library separately rather you can use the instructions below to build this library.


### Dependencies
To compile the library we require a C++ compiler and CMake 3.5.1 or later, and the following libraries:
- GMP
- openSSL

### Steps to build supra-bicycl
The library is tested on Ubuntu and Macos with Clang 13.0.0 and OpenSSL 3.0.0.

1.  Clone this repository.

2. Verify you have clang and cmake (version>=3.5.1) installed. The library is tested with clang 13.0.0. Also verify that GMP and openSSL are installed on your system. 

    On Ubuntu the required dependencies can be installed using:  
    `apt install g++ libgmp-dev libssl-dev cmake`

    On Macos, if cmake is not installed, you can install it using:
    ```
     brew install cmake
    ```

    On Macos, Ritual library used to generate rust binding does not seem to compile with Apple Clang. So on Macos it is important to install clang either using brew or manually by using the     [source code](https://github.com/llvm/llvm-project/releases/download/llvmorg-13.0.0/llvm-project-13.0.0.src.tar.xz "source code"). Here is an example of installing clang using homebrew:

   ```
    brew install llvm
   ```
    After installation, you can set the environment variables as below to temporarily set the compiler to newly installed clang for the current terminal session, for example as:  
    ```
    export CC="/opt/homebrew/opt/llvm/bin/clang"
    export CXX="/opt/homebrew/opt/llvm/bin/clang++"
    ```

    **NOTE**: Donot overwrite the Apple clang with the newly installed clang. After installation, make sure check that `clang` command on your macos still
    refer to apple version. For example:
    ```
    clang -v
    Apple clang version 15.0.0 (clang-1500.1.0.2.5)
    Target: arm64-apple-darwin23.2.0
    Thread model: posix
    InstalledDir: /Library/Developer/CommandLineTools/usr/bin
    ```

4. Ensure that GMP is installed by locating /usr/local/include/gmp.h file. 
   If GMP not installed in /usr/local, build from source using Apple pre-installed clang as below:
   Download [gmp source code](https://gmplib.org/).
   ```
   tar -xvf gmp-x.x.x.tar.xz  # Replace x.x.x with the actual version number
   cd gmp-x.x.x
   ./configure --prefix=/usr/local
   make
   make check  # Optional: Run tests
   sudo make install
   ```
5. For openSSL, to ensure the ritual library has access to openSSL headers and lib you need to set the following environment variables:
      
    ```
    export RITUAL_INCLUDE_PATH="/opt/homebrew/opt/openssl@3/include/"
    export LIBRARY_PATH="/opt/homebrew/opt/openssl@3/lib"
    export RITUAL_LIB_PATH=$LIBRARY_PATH
    ```
    Change the paths as needed per your installation. Note this only sets the environment variables temporarily for the current terminal session.

6. You can now build the rust library using:
   ```
   cargo clean
   cargo build --release
   ```
