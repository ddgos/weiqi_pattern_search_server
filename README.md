# Weiqi Pattern Search Server

The Weiqi Pattern Search Server is a tool to search for go learning material
via positions.

## Setup

 1. Clone the repository
 2. Setup the database
  1. Create the database file '''./dbs/patterns.sqlite'''
  2. Create a table '''patterns''' with columns '''lec_id INT''' and '''pat VARCHAR(400)'''
 3. Run the server using '''cargo run''' (if you don't have cargo, [get it](https://rustup.rs))
