import asyncio
import websockets
import json
import ctypes
import os
import sys
import threading
import time
from http.server import SimpleHTTPRequestHandler
import socketserver

# تنظیمات
LIB_PATH = "./libaeon_core.so"
WS_PORT = 8765
HTTP_PORT = 8000
SEED = 123456789

connected_clients = set()
aeon_lib = None

def load_rust_core():
    global aeon_lib
    if not os.path.exists(LIB_PATH):
        print("[ERROR] .so file not found!")
        return False
    try:
        aeon_lib = ctypes.CDLL(LIB_PATH)
        aeon_lib.aeon_start_node.argtypes = [ctypes.c_longlong]
        aeon_lib.aeon_start_node.restype = ctypes.c_int
        try:
            aeon_lib.aeon_send_message.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
            aeon_lib.aeon_send_message.restype = ctypes.c_int
        except:
            pass
        print("[CORE] Library loaded.")
        return True
    except Exception as e:
        print(f"[ERROR] Failed to load lib: {e}")
        return False

def start_log_capture():
    r_fd, w_fd = os.pipe()
    os.dup2(w_fd, sys.stdout.fileno())
    os.dup2(w_fd, sys.stderr.fileno())
    
    def reader():
        with os.fdopen(r_fd, 'r', errors='replace') as pipe:
            while True:
                line = pipe.readline()
                if not line: break
                clean_line = line.strip()
                if clean_line:
                    asyncio.run(broadcast_log(clean_line))
    t = threading.Thread(target=reader, daemon=True)
    t.start()

async def broadcast_log(message):
    if not connected_clients: return
    payload = json.dumps({"type": "log", "content": message})
    await asyncio.gather(*[client.send(payload) for client in connected_clients], return_exceptions=True)

async def ws_handler(websocket):
    connected_clients.add(websocket)
    try:
        async for message in websocket:
            data = json.loads(message)
            if data['action'] == 'send':
                peer = data['peer']
                msg = data['msg']
                print(f"[BRIDGE] Sending to {peer}: {msg}")
                if aeon_lib:
                    c_peer = ctypes.c_char_p(peer.encode('utf-8'))
                    c_msg = ctypes.c_char_p(msg.encode('utf-8'))
                    aeon_lib.aeon_send_message(c_peer, c_msg)
    except: pass
    finally: connected_clients.remove(websocket)

async def start_ws_server():
    print(f"[WS] Server on {WS_PORT}")
    async with websockets.serve(ws_handler, "0.0.0.0", WS_PORT):
        await asyncio.Future()

def run_node():
    if aeon_lib: aeon_lib.aeon_start_node(SEED)

def start_http_server():
    class QuietHandler(SimpleHTTPRequestHandler):
        def log_message(self, format, *args): pass
    with socketserver.TCPServer(("", HTTP_PORT), QuietHandler) as httpd:
        print(f"[HTTP] Web UI at http://localhost:{HTTP_PORT}")
        httpd.serve_forever()

if __name__ == "__main__":
    if load_rust_core():
        start_log_capture()
        threading.Thread(target=start_http_server, daemon=True).start()
        threading.Thread(target=run_node, daemon=True).start()
        try:
            asyncio.run(start_ws_server())
        except KeyboardInterrupt:
            print("\n[SYSTEM] Shutdown.")
