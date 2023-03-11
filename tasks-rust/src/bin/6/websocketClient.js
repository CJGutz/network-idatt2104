const net = require('net');

// Simple HTTP server responds with a simple WebSocket client test
const httpServer = net.createServer((connection) => {
    connection.on('data', () => {
        let content = `<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
  </head>
  <body>
    WebSocket test page
    <input type="text" id="message" />
    <div id="wsMessages"></div>
    <script charset="utf-8">
      let ws = new WebSocket('ws://localhost:8000');
      ws.onmessage = event => {
        console.log("event data", event.data)
        document.getElementById('wsMessages').innerHTML += event.data + '<br />';
      };
      ws.onopen = () => {
        console.log('Connected to server');
      };
      document.getElementById('message').addEventListener('keyup', (event) => {
        if (event.key === "Enter") {
          ws.send(event.target.value);
        }
      });
    </script>
  </body>
</html>
`;
        connection.write('HTTP/1.1 200 OK\r\nContent-Length: ' + content.length + '\r\n\r\n' + content);
    });
});
httpServer.listen(3000, () => {
    console.log('HTTP server listening on port 3000');
});
httpServer.onopen = () => {
    console.log('HTTP server connected');
}
