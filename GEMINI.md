# Project: S3-Like Object Store in Rust

## Project Overview

This project is a Rust-based implementation of an S3-like object store. The goal is to create a storage system that can efficiently handle both small and large files. Small files (<= 10KB) are packed together into larger segment files to save space and reduce overhead, while larger files are stored individually.

The system is designed to use `sled`, a Rust embedded database, for managing metadata, and `uuid` for assigning unique IDs to objects. The project also plans to expose an HTTP API for uploading, downloading, and listing objects.

## Requirements
1. HTTP PUT API for end client to upload a object, end client provide the object name and the prefixes

2. HTTP GET API for end client to download an object, end client provide the object URI including the prefixes and actual object name

3. A suitable HTTP method provided to end client to list all the object when given a prefix (directory), objects share same prefixes should all be included in the  response 

4. Physical object store component which in charge of storing object in the disk 

5. Metadata store component in charge of keeping track of the object metadata, so that when end client submit a download request by prefixes and file names, it know where it is stored in the actual address. Preferably using sled as it is native rust package and the key-value store is suitable for list operations

## Notes
This is going to be a learning project for me to understand how modern storage are implemented and I want to learn basic knowledge of how modern storages are implemented. Also I want to learn Rust the programming language.

It is not a full scale production ready service, but more so a interactable local service provide basic functionalities

## Building and Running

This is a standard Rust project. You can use the following `cargo` commands to build, run, and test the project:

*   **Build:**
    ```bash
    cargo build
    ```

*   **Run:**
    ```bash
    cargo run
    ```

*   **Test:**
    ```bash
    cargo test
    ```

*   **Check:**
    ```bash
    cargo check
    ```

## Development Conventions

*   As this is the learning project, would like to be advised with Rust's best coding practices
*   The code is organized into modules under the `src` directory.
*   The `design_doc.md` file provides a high-level overview of the architecture and should be consulted before making significant changes.

## Notes
