const express = require("express");
const fileUpload = require("express-fileupload");

// wasm
const { wasm_infer }    = require("../pkg/AMLVM.js");
const { wasm_hello }    = require("../pkg/AMLVM.js");
const { wasm_fit_draw } = require("../pkg/AMLVM.js");

const {
  performance
} = require("perf_hooks");
const url = require("url");
const fs = require("fs");

var data_model = fs.readFileSync("mobilenet_v2_1.4_224_frozen.pb");
var labels = [];
fs.readFileSync("imagenet_slim_labels.txt", "utf-8")
  .split(/\r?\n/)
  .forEach(function (line) {
    labels.push(line);
});

const app = express();
const host = "0.0.0.0";
const port = 8080;

app.use(express.static(__dirname));
app.use(fileUpload());

app.get("/", (req, res) => res.redirect("/index.html"));

app.get("/ml", (req, res) => res.redirect("/ml.html"));

app.get("/kc", (req, res) => res.redirect("/kc.html"));

app.get("/hello", function(req, res) {
    console.log("Received /hello request");

    const queryObject = url.parse(req.url,true).query;

    if (!queryObject['name']) {
        res.end(`Please use command curl http://localhost:8080/hello?name=MyName \n`);
    } else {
        res.send(wasm_hello(queryObject["name"]));
    }
});

app.post("/kc/draw", function (req, res) {
    console.log("Received /kc/draw request");

    var svg = wasm_fit_draw(
        parseInt(req.body.points_num),
        parseInt(req.body.centers_num),
        // parseFloat(req.body.pxmi),
        // parseFloat(req.body.pxma),
        // parseFloat(req.body.pymi),
        // parseFloat(req.body.pyma),
        parseInt(req.body.pxmi),
        parseInt(req.body.pxma),
        parseInt(req.body.pymi),
        parseInt(req.body.pyma),
        800,
        400,
        50,
        req.body.title
    );
    res.send(svg)
});

app.post("/ml/infer", function(req, res) {
    console.log("Received /ml/infer request");

    if (!req.files || Object.keys(req.files).length === 0) {
        return res.status(400).send("No files were uploaded.");
    }

    console.log("Received /ml/infer request");

    var datetime = new Date();
    console.log("\n" + datetime);
    console.log(
        "Received " +
        req.files.image_file.name +
        " with size: " +
        req.files.image_file.size
    );

    let image_file = req.files.image_file;

    var t_start = performance.now();
    var result = JSON.parse(wasm_infer(data_model, image_file.data, 224, 224));
    var t_end = performance.now();
    var datetime = new Date();
    console.log(datetime);

    res.send(
        labels[result[1] - 1] +
        "<br>confidence: " +
        result[0] +
        "<br>Recognize time: " + (t_end - t_start)
    );
});

app.listen(port, host, () =>
    console.log(`AMLVM at http://${host}:${port}\n`)
);
