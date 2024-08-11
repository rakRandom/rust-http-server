# HTTP Server (in Rust!)

### A server based on HTTP made with Rust from Scratch. Supports multithreading and (only) GET requests.

## How to use:

Make sure that you have `cargo` and `rustc` installed in your computer

Run `cargo build --release` at the cmd to build an executable

Run `.\target\release\http-server.exe` to start the server

Access `http://<you_local_address>:7878` to see the index page

> To download a file, access the endpoint `/file`. Example: `/file/filename.ext/p=<password>`
> This project was only tested on Windows 10.

## Types of status code:

200 OK: Success - The action was successfully received, understood, and accepted

404 NOT FOUND: Client Error - The request contains bad syntax or cannot be fulfilled

## Based on:

- https://doc.rust-lang.org/book/ch20-01-single-threaded.html

- https://doc.rust-lang.org/book/ch20-02-multithreaded.html

- https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html

- Lots of research to improve even more

## Licence

This project is currently under a [LICENSE](LICENSE)
