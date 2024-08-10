import socket
import json
import uuid
import time


def create_message(queue_name, sender, receiver, event_type, data):
    message_id = str(uuid.uuid4())
    timestamp = int(time.time())
    correlation_id = str(uuid.uuid4())
    header = {
        "message_id": message_id,
        "timestamp": str(timestamp),   
        "correlation_id": correlation_id,
        "token": "tester",
        "x_key": "12345789"
    }
    message = {
        "queue_name": queue_name,
        "type": "add",
        "command": "publish",
        "header": header,
        "payload": {
                "event_type": event_type,
                "data": data
            },
        "metadata": {
            "retry_count": 0,
            "ttl": 3600,
            "tags": []
        },
        "sender": None
    }
    return message

def send_message(host, port, message):
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.connect((host, port))
            print(f"Connected to {host}:{port}")
            for i in range(1):
                message_json = json.dumps(message)
                sock.sendall(message_json.encode('utf-8'))
                print(f"Sent: {message_json}")
                received_data = sock.recv(1024)
                print(f"Received: {received_data.decode('utf-8')}")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == '__main__':
    HOST, PORT = 'localhost', 5672
    
    MESSAGE = create_message(
        "test_queue",
        sender="producer",
        receiver="consumer",
        event_type="test_event",
        data={"key": "value"}
    )
    send_message(HOST, PORT, MESSAGE)