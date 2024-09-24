# Weiqi Pattern Search Server

The Weiqi Pattern Search Server is a tool to search for go learning material
via positions.

## Setup

1. Clone the repository
2. Setup the database
    1. Create the database file `./dbs/patterns.sqlite`
    2. Create a table `patterns` with columns `(lec_id INT, pat VARCHAR(400))`

## Running the Server

Using either of these methods should create a server accessible from `127.0.0.1:8080`.

### Using Cargo

Run the server using `cargo run` (if you don`t have cargo, [get it](https://rustup.rs))

### Using Docker

You will need Docker as well as a running Docker daemon for this method.

1. Build the docker image `docker build .` (note the period to specify *this* directory)
2. Figure out the docker image id using `docker images`
3. Run the image in a container using `docker run -ti --net=host $IMAGE_NAME`

