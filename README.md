## About
This project aims to replace the "Neelix" program in the IT support of the University of Bamberg. Neelix was written around 2015-2017, and the source code seems to be lost. You need to have Java installed and set a variable for JAVA_HOME. The database server uses an out of supported Linux system.

Reelix wants to redo all this. Written in Rust and build with [Slint](https://slint.dev/) as the UI Framework. Reelix should run on every desktop OS after compilation. Officially supported are (yet) only Linux and Windows.


# Things to add

 On the Record panel
- [x] Replace Combobox with ComboboxScroll
- [ ] Operating System selection in sertent categories (working on backend)
- [ ] A text box for editional info (lineedit + no backend)
- [ ] Select time for submitting (can't select yet comming soon)
- [ ] Select place were the issue happend 
- [ ] Select the number of thickets you want to create at once
- [x] Submitt button (early stages)

On the Request-Log panel
- [ ] Dynamicly Created Table sheet
- [ ] show the latest 20 entrys 
- [ ] ability to load more older once with a extra button
- [ ] ability to remove accidely created once

On the Statistics panel
- [ ] Show statisc form the last 7 days
- [ ] optional to ask for a specific date range
- [ ] optional scroll back to the 7 days before the current week

On the Analyzes panel
- [ ] Export sql table to csv or html
- [ ] need to have the ability to select the date range

On the Categorie Management panel
- [x] To Create Categories(Typs)
- [x] To Delete Categories(Typs)

On the Settings panel
- [ ] ability to set a database timeout
- [ ] ability to set a databse server with IPv4 address (in the future ipv6 and maybe domain name)
- [ ] ability to set a databse port
- [ ] ability to set a databse name
- [ ] ability to set a databse userame
- [ ] ability to set a databse password
- [ ] check if it would work
- [x] dark/light mode switch
- [ ] dark/light mode switch state
- [ ] theme switcher (fluent, cosmic, cupertino)

A Info panel
- [x]  show some stuff about the application 

### Independend / Global Issues

CoboboxScroll
- [x] make the popup solid insted of transperent
- [x] Scrolling works
- [x] Theme works on it
- [ ] ComboboxScroll support key inputs
- [ ] ComboboxScroll tab support

more things will be added in the future

## Things done
- Sql basics, add categories, remove categories, ask for existing categories(see cmd branches and typetesting)
- Dynamicly create categories (in Gui) see branch combobox



### Things Reelix needs to can:
Typs must be removeable and addable
Typs must be centrilysed in another table
clients must use typs form the cenrilysed location

- Config.toml file support
- build binarys for Windows and Linux
- Basicly have the same functionalatiy as Neelix and more


# How to use

### The easy way 
Download the bin file for your Oporating System of Choice

### THE HARD WAY
1. You have to install  ```rustc``` and ```cargo``` [(Rust Getting Started Guide)](https://www.rust-lang.org/learn/get-started)
2. Go to [the tutorial](https://releases.slint.dev/1.5.1/docs/tutorial/rust/introduction)
there you have to install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)

```
cargo install cargo-generate
```
3.   Clone this Repo

    git clone https://gitlab.rz.uni-bamberg.de/ba4ju8/reelix/
 
4. Cd into it    
```
cd ./reelix
 ```
5. Smash your head against the wall
6. Build the binary
```
cargo build
```


If this doesn't work good luck 

# Setting up the database
Install mariadb or mysql on a Linux system of your choice
Recommended  latest stable release of Debian currently 12

From : [Digitalocean](https://www.digitalocean.com/community/tutorials/how-to-install-mariadb-on-debian-11)
I know it's for Debian 11 but it also works on 12

```
sudo apt update
sudo apt install mariadb-server
sudo mysql_secure_installation
```

Check if it worked
```
sudo systemctl status mariadb
```

Go into mariadb
```
sudo mariadb
```

Create a Database
```
CREATE DATABASE your_database_name;
```

Create a User with password
```
CREATE USER 'your_database_username'@'localhost' IDENTIFIED BY 'your_database_password';
```

Then grant the user all permissions on the database (can be insecure)
```
GRANT ALL PRIVILEGES ON your_database_name.* TO 'your_database_username'@'%';
```

Then grant the user acses to the the database from every host (can be insecure)
```
GRANT ALL PRIVILEGES ON *.* TO 'your_database_username'@'%' IDENTIFIED BY 'your_database_password' WITH GRANT OPTION;
```
