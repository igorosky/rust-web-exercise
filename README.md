# Rust web exercise

## Build
In order to build simply use docker (or podman) buildx. Assuming you are in the root directory of the project use for example:
```bash
docker buildx build -t rust-web-exercise .
```

> Note change `docker` to `podman` if you are using podman

## Run
To run the application in the simplest form simply use:
```bash
docker run -p 3000:3000 rust-web-exercise
```

> Note change `docker` to `podman` if you are using podman

Application will be available at [`http://localhost:3000/home`](http://localhost:3000/home)

## Environmental variables
 - `RUST_BACKTRACE` - for [std::backtrace](https://doc.rust-lang.org/std/backtrace/index.html)
 - `RUST_LOG` - for [tracing](https://docs.rs/tracing/latest/tracing/) crate
 - `DATABASE_URL` - path to sqlite3 database file
 - `UPLOAD_DIRECTORY` - path to directory to which images will be saved
 - `STATIC_FILES_DIRECTORY` - path to directory where static files are located (recommended not to change)
 - `UPLOAD_BUFFER_SIZE` - size of a buffer for image saving in bytes (has to be at least 8 bytes if less 8 will be used)
 - `MAX_BODY_SIZE` - Maximum size of a request body in bytes
 - `ADDRESS` - address on which the server will listen (default: `0.0.0.0:3000`)
