const express = require("express");
const fileUpload = require("express-fileupload");
const { infer } = require("../pkg/ssvm_recognition.js");
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

app.get("/hello", function(req, res) {
    const queryObject = url.parse(req.url,true).query;
    res.send("hello, " + queryObject["name"]);
});

app.post("/machine_learning/infer", function(req, res) {
    if (!req.files || Object.keys(req.files).length === 0) {
        return res.status(400).send("No files were uploaded.");
    }

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
    var result = JSON.parse(infer(data_model, image_file.data, 224, 224));
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
    console.log(`ssvm_recognition at http://${host}:${port}\n`)
);
