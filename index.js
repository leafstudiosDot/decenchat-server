const express = require('express')
const app = express()
const { networkInterfaces } = require('os');

app.get("/", (req, res) => {
    res.send("Please open this address with Decensha Client")
})

app.listen(7810, () => {

    const nets = networkInterfaces();
    const netresults = Object.create(null);

    for (const name of Object.keys(nets)) {
        for (const net of nets[name]) {
            const familyV4Value = typeof net.family === 'string' ? 'IPv4' : 4
            if (net.family === familyV4Value && !net.internal) {
                if (!netresults[name]) {
                    netresults[name] = [];
                }
                netresults[name].push(net.address);
            }
        }
    }

    console.log("Your Decensha Server port for development is http://" + netresults["en0"][0] + ":7810")
})