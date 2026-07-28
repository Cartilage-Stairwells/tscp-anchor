from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import os

from .continuity_event import create_event
from .evaluator import evaluate
from .receipt import generate
from .replay import replay
from .ledger import record


LEDGER_FILE = "data/event_ledger.jsonl"


class Handler(BaseHTTPRequestHandler):

    def send_json(self, data):

        self.send_response(200)
        self.send_header(
            "Content-Type",
            "application/json"
        )
        self.end_headers()

        self.wfile.write(
            json.dumps(data).encode()
        )


    def do_GET(self):

        if self.path == "/ledger":

            events = []

            if os.path.exists(LEDGER_FILE):

                with open(LEDGER_FILE) as f:
                    for line in f:
                        events.append(
                            json.loads(line)
                        )

            self.send_json({
                "count": len(events),
                "events": events
            })

        else:

            self.send_json({
                "service": "Continuity Runtime Alpha",
                "status": "online"
            })


    def do_POST(self):

        length = int(
            self.headers["Content-Length"]
        )

        body = self.rfile.read(length)

        data = json.loads(body)

        event = create_event(data)

        result = evaluate(event)

        receipt = generate(
            event,
            result
        )

        record(
            event,
            receipt
        )

        replay_result = replay(event)

        self.send_json({
            "receipt": receipt,
            "replay": replay_result
        })


server = HTTPServer(
    ("0.0.0.0",8000),
    Handler
)

print(
    "Continuity Runtime Alpha listening on :8000"
)

server.serve_forever()
