# zecpaperwallet
zecpaperwallet is a Zcash Sapling paper wallet generator that can run completely offline. You can run it on an air-gapped computer to generate your shielded z-addresses, which will allow you to keep your keys completely offline. 

# Download
zecpaperwallet is available as pre-built binaries from our [release page](https://github.com/adityapk00/zecpaperwallet/releases). Download the zip file for your platform, extract it and run the `./zecpaperwallet` binary. 

# Generating wallets
To generate a zcash paper wallet, simply run `./zecpaperwallet`

You'll be asked to type some random characters that will add entropy to the random number generator. Run with `--help` to see all options

## Saving as PDFs
To generate a zcash paper wallet and save it as a PDF, run
`./zecpaperwallet -z 3 --format pdf zecpaper-output.pdf`

This will generate 3 shielded z-addresses and their corresponding private keys, and save them in a PDF file called `zecpaper-output.pdf`

# Compiling from Source
zecpaperwallet is built with rust. To compile from source, you [install Rust](https://www.rust-lang.org/tools/install). Basically, you need to:
```
curl https://sh.rustup.rs -sSf | sh
```
Chekout the zecpaperwallet repository and build the CLI
```
git clone https://github.com/adityapk00/zecpaperwallet.git
cd zecpaperwallet/cli
cargo build --release
```

The binary is available in the `target/release` folder.

## Run without network
If you are running a newish version of Linux, you can be doubly sure that the process is not contacting the network by running zecpaperwallet without the network namespace.

```
sudo unshare -n ./target/release/zecpaperwallet
```
`unshare -n` runs the process without a network interface which means you can be sure that your data is not being sent across the network. 


## Help options
```
USAGE:
    zecpaperwallet [FLAGS] [OPTIONS] [output]

FLAGS:
    -e, --entropy    Provide additional entropy to the random number generator. Any random string, containing 32-64
                     characters
    -h, --help       Prints help information
    -n, --nohd       Don't reuse HD keys. Normally, zecpaperwallet will use the same HD key to derive multiple
                     addresses. This flag will use a new seed for each address
    -t, --testnet    Generate Testnet addresses
    -V, --version    Prints version information

OPTIONS:
    -f, --format <FORMAT>         What format to generate the output in [default: json]  [possible values: pdf, json]
    -z, --zaddrs <z_addresses>    Number of Z addresses (Sapling) to generate [default: 1]

ARGS:
    <output>    Name of output file.
```
