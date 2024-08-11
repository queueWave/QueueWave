import socket
import json
import uuid
import time

def create_message(queue_name):
    message_id = str(uuid.uuid4())
    timestamp = int(time.time())
    correlation_id = str(uuid.uuid4())
    header = {
        "message_id": message_id,
        "timestamp": str(timestamp),   
        "correlation_id": correlation_id,
        "token": "tester",
        "x_key": "123456789"
    }
    send_message = {
        "queue_name": queue_name,
        "type": "get",  
        "command": "consume",
        "header": header,
        "sender": {
            "user": "tester",
            "service": "test-consume",
            "name": "tester"
        }
    
    }
    return send_message


def receive_message(host, port):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((host, port))
        print(f"Connected to {host}:{port}")
        while True:
            message_json = json.dumps(create_message("test_queue"))
            sock.sendall(message_json.encode('utf-8'))
            received_data = sock.recv(1024)
            if received_data.decode('utf-8') != "No message available":
                try:
                    received_dict = json.loads(received_data)
                    if 'data' in received_dict.get('payload', {}):
                        print(f"Data: {received_dict['payload']['data']}")
                except json.JSONDecodeError:
                    print("Received invalid JSON data")
if __name__ == '__main__':
    HOST, PORT = '127.0.0.1', 5672
    receive_message(HOST, PORT)