import signal
from websocket_example import websocket_example


try:
    server = websocket_example.spawn_websocket_server()
    print(server)
except KeyboardInterrupt:
    print("Done")
    exit()
