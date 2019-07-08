## ZecPaperWallet Web
The paper wallet generator [is hosted at paper.zecwallet.co](https://paper.zecwallet.co).

This is a web version of the Zec Sapling Paper Wallet generator. It's mainly for illustrative purposes. If you want to generate a serious offline paper wallet, you should run [zecpaperwallet](https://github.com/adityapk00/zecpaperwallet) offline. 

### Running locally
You can run the web wallet locally. You need to install the following first:
1. Rust 1.32+
2. wasm-build
3. nodejs / npm 

```
cd zecpaperwallet/web
wasm-pack build
cd www
npm install
npm run start
```

This will start a local web server at `localhost:8080` where you can access the paper wallet