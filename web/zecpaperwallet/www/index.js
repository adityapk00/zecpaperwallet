global.jQuery = require('jquery');
require('bootstrap');
var QRCode = require('qrcode')

import * as wasm from "zecpaperwallet";

let w = JSON.parse(wasm.greet());
console.log(w[0]);
jQuery("body").append(w[0]["address"]);

QRCode.toCanvas(document.getElementById("qr1"), w[0]["address"]);