import http.server
import socketserver
from http.server import HTTPServer, BaseHTTPRequestHandler
from io import BytesIO
from urllib.parse import parse_qs

PORT = 8089
Handler = http.server.SimpleHTTPRequestHandler

class SimpleHTTPRequestHandler(BaseHTTPRequestHandler):

        def do_GET(self):
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'Hello, world!')
        def do_POST(self):
            content_length = int(self.headers['Content-Length'])
            body = self.rfile.read(content_length)
            parsed = parse_qs(body)
            print(parsed)
            self.send_response(200)
            self.end_headers()
            response = BytesIO()
            response.write(b'This is POST request. ')
            response.write(b'Received: ')
            response.write(body)
            self.wfile.write(response.getvalue())


with socketserver.TCPServer(("", PORT), SimpleHTTPRequestHandler) as httpd:
        print("serving at port", PORT)
        httpd.serve_forever()
