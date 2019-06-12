# zecpaperwallet
zecpaperwallet is a Zcash Sapling paper wallet generator that can run completely offline. You can run it on an air-gapped computer to generate your shielded z-addresses, which will allow you to keep your keys completely offline. 

# Compiling
zecpaperwallet-cli is built with rust. To compile from source, you [install Rust](https://www.rust-lang.org/tools/install). Basically, you need to just:
`curl https://sh.rustup.rs -sSf | sh`

```
git clone https://github.com/adityapk00/zecpaperwallet.git
cd zecpaperwallet/cli
cargo build --release
```
# Generating wallets
To generate a zcash paper wallet, simply run
`./target/release/zecpaperwallet`

You'll be asked to type some random characters that will add entropy to the random number generator. 

## Saving as PDFs
To generate a zcash paper wallet and save it as a PDF, run
`./target/release/zecpaperwallet -z 3 --format pdf zecpaper-output.pdf`

This will generate 3 shielded z-addresses and their corresponding private keys, and save them in a PDF file called `zecpaper-output.pdf`

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
