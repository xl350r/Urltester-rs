# UrlTester-rs

A Urltester written in Rust. Based off of a ruby script I wrote a year ago to test proxy and firewall filters for websites. Included is a formatted version of the [majestic million](https://majestic.com/reports/majestic-million). This repository is provided as freeware. If you like it let me know, if you have suggestions for improvement or change also let me know. 

### Note

If stuff doesn't work let me know, i've tested it to work on my network but it may not work on yours. Some of the ways I've implemented aspects of the code may be unoptimal as I'm (A) not a software developer by trade, and (B) I am still learning serveral aspects of coding on this level typically coding/scripting with ruby, python, bash or powershell.
 
### Dependacies
* reqwest = "0.9.22"
* threadpool="1.7.1"
* clap = "2.33.0"

### Usage

```
UrlTester-rs 0.1.2
Daniel Hoberecht
Test Web Proxies from list of addresses

USAGE:
    UrlTester-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -P, --pass <Pass>          Basic Proxy Auth Password
    -p, --proxy <Proxy>        Proxy
    -t, --threads <Threads>    number of thread to use
    -u, --user <User>          Basic Proxy Auth Username
    -f, --file <file>          File of urls to use
```

### Example

```sh
$> UrlTester-rs -t 10 --proxy "http://localhost:8000" -f urls.lst
```
