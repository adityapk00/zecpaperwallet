global.jQuery = require('jquery');
require('bootstrap');
var QRCode = require('qrcode')

import * as wasm from "zecpaperwallet";

var address_number = 0;

function add_section(wallet_item) {
    let htmls = `
        <div class="row address-section">
            <div class="col-sm-9" style="word-break: break-word;">
                <h1> Address (Sapling) </h1>
                <p class="fixed-width"> ${wallet_item["address"]} </p>
            </div>
            <div class="col-sm-3">
                <canvas id="qrcode_addr_${address_number}"></canvas>
            </div>
        </div>
    `;
    jQuery("#wallet").append(htmls);
    QRCode.toCanvas(document.getElementById("qrcode_addr_"+address_number), 
        wallet_item["address"], {
            scale : 5.5
        });


    let pk_section = `
        <div class="row pk-section">
            <div class="h-dashed"></div>
            <div class="col-sm-3">
                <canvas id="qrcode_pk_${address_number}"></canvas>
            </div>
            <div class="col-sm-9" style="word-break: break-word;">
                <h1> Private Key </h1>
                <p class="fixed-width"> ${wallet_item["private_key"]} </p>
                <br/>
                <h2> Address </h2>
                <p class="fixed-width"> ${wallet_item["address"]} </p>
                <code> HD Key: ${wallet_item["seed"]["HDSeed"]}, path: ${wallet_item["seed"]["path"]} </code>
            </div>
        </div>
        <div class='h-divider'></div>
    `;
    
    jQuery("#wallet").append(pk_section);
    QRCode.toCanvas(document.getElementById("qrcode_pk_"+address_number), 
        wallet_item["private_key"], {
            scale: 3.5
        });

    address_number++;
}

let w = JSON.parse(wasm.get_wallet());
console.log(w);

w.forEach(wallet_item => {
    add_section(wallet_item); 
});
 

