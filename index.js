const express = require('express')
const app = express()

app.get("/", (req, res) => {
    res.send("Please open this address with Decensha Client")
})

app.listen(7810, () => {
    console.log("Your Decensha Server port for development is 7810")
})