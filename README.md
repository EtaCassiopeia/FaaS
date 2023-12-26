# Function as a Service (FaaS) in Rust

This project implements a Function as a Service (FaaS) system in Rust, comprising a server and a client. It allows for the uploading and invocation of dynamic libraries as functions.

## Building the dynamic library
This project contains a sample dynamic library that can be used to test the FaaS system. To build the dynamic library, run the following command:
```bash
cd udfs
cargo build --release
```
This will generate a dynamic library file in the `udfs/target/release` directory. The dynamic library file will have the same name as your package, as specified in your `Cargo.toml` file (`libadd.dylib`).

In MacOS, the dynamic library file will have the extension `.dylib`. In Linux, the extension will be `.so`.

As the next step, you can upload the dynamic library to the FaaS server and invoke functions from it:

## Setting up the FaaS Server

1. **Create an Upload Directory**:
   In the root directory of the project, create a folder named `uploads` to store the uploaded dynamic libraries.

2. **Start the Server**:
   Run the following command to start the FaaS server:
   ```bash
   cargo run --bin server
   ```

## Using the FaaS Client

The client supports uploading dynamic libraries and invoking functions from them.

1. **Uploading a Dynamic Library**:
   To upload a library, use:
   ```bash
   cargo run --bin client upload <path_to_dynamic_library>
   ```
   Replace `<path_to_dynamic_library>` with the actual path to your dynamic library file.

   Example:
   ```bash
   cargo run --bin client upload /path/to/libadd.dylib
   ```

2. **Invoking a Function**:
   To invoke a function from an uploaded library, use:
   ```bash
   cargo run --bin client invoke <library_name> <optional_args>
   ```
   Replace `<library_name>` with the name of the uploaded library. `<optional_args>` is a string of arguments separated by commas, if applicable.

   Example:
   ```bash
   cargo run --bin client invoke libadd 1,2
   ```