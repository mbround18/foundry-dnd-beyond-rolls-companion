import express from "express";
import cors from "cors";
import * as https from "https";

const port = 8745;

/**
 *
 * @param {Request} req
 * @param {Response} res
 */
const requestListener = function (req, res) {
  res = cors(req, res);
};

const app = express();

const corsOptions = {
  origin: "*",
  optionsSuccessStatus: 200,
};

app.use(cors(corsOptions));

app.get("/healthz", (req, res) => {
  res.status(200);
  res.send("OK");
})

app.get("/", (req, res) => {
  let authorization = (
    (req &&
      req.headers &&
      (req.headers["authorization"] || req.headers["Authorization"])) ||
    ""
  ).replace("Bearer ", "");

  if (!authorization) {
    res.status(401);
    res.send("No Authorization header was found");
    return;
  }

  //request token from DDB
  let options = {
    host: "auth-service.dndbeyond.com",
    path: "/v1/cobalt-token",
    port: "443",
    method: "POST",
    headers: { cookie: "CobaltSession=" + authorization },
  };

  let callback = (response) => {
    let str = "";
    response.on("data", function (chunk) {
      str += chunk;
    });

    response.on("end", function () {
      //return token to foundry
      res.status(200);
      res.send(str);
    });
  };

  //request socket token from DDB
  https.request(options, callback).end();
});

app.listen(port, "0.0.0.0", undefined, () => {
  console.log(`Waiting for Foundry requests http://127.0.0.1:${port}`);
});
