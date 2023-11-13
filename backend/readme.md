# PROJECT SETUP

- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh {install cargo and rust}
- or u can use rustup init {installing rust}
- cargo new rust_rocket_project
- "rustup toolchain install nightly" //chk u have or not
- rustup override set nightly
- cd rust_rocket_project
- cargo build
- cargo run

- if u change yur dependggency then must "cargo update" then cargo build
- things like spring, dont need install pakages extra just add in Cargo.toml and hit "cargo update"

- there is another folder named ROCKET_RUST_Basic must study this folder to understand moudle system

# DATABSE SETUP

- here used sqlite (instal and just extract and set the path not need installation)
- https://www.youtube.com/watch?v=L3FwRRx6bqo
- go cmd hit sqlite (it will says u that set up sqlite or not)
- disel a thing that can handle mongodb/postgres/mysql OMR for rust database system
- set up dependency for sqlite and dotenv
- cargo install diesel_cli --no-default-features --features "sqlite-bundled" //must chek u already have or not because it is globally installation

# BASIC

a. create project
b. switch to nightly
c. chek diesel is install or not
d. go to migration folder create table(inside up file)
e. here database is used sqlite3
f. for sqlite3 installation see amit think video
. for extra information i have add ss of sqlite that how to create and see
. https://diesel.rs/guides/getting-started.html (very important)
.

filters are called here request guard and also has middleware concept
can be done as node.js style but two things this project hasnt is get data and filters

The dependecy are used here is best

# CREATE DATABSE WITH DIESEL

- make .env
- use DATABASE_URL=urdbname.db
- hit deisel setup // chk u has deisel has not not
- then u have migration folder
- inside folder has folder then up and down two file
- in up file create table
- in down file drop file
- hit run migration
- why i choose becasue diesel is easy for migration

and of course u can make model for inserting data as struct

# UNDERSTANDING MOUDLE AND EXPORT AND IMPORT IMPORTANT

- For this i have a project name "TestRocket"

# Filter ware Done

- Fllw the prject; filters are called here guard
- but here middle ware called inside the methd parameter
- fllw the hello method

# MIDDLEWARE

- CROS are used as middleware attach in applicatin level
