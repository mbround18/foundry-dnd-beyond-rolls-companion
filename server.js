const http = require("http");
const https = require("https");
const host = 'localhost';
const port = 8745;

const requestListener = function (req, res) {
    //set cors
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Request-Method', '*');
    res.setHeader('Access-Control-Allow-Methods', 'OPTIONS, GET');
    res.setHeader('Access-Control-Allow-Headers', '*');

    //request token from DDB
    var options = {
        host: 'auth-service.dndbeyond.com',
        path: '/v1/cobalt-token',
        port: '443',
        method: 'POST',
        headers: {'cookie': 'CobaltSession=' + req.url.slice(1)}
    };

    callback = function(response) {
        var str = ''
        response.on('data', function (chunk) {
            str += chunk;
        });

        response.on('end', function () {
            //return token to foundry
            console.log(str)
            res.writeHead(200);
            res.end(str);
        });
    }

    //request socket token from DDB
    var req = https.request(options, callback);
    req.end();
};

const server = http.createServer(requestListener);
server.listen(port, host, () => {
    console.log(`Waiting for Foundry requests http://${host}:${port}`);
});
