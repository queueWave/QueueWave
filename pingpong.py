import socket
import json
import uuid
import time

def create_publish_message(queue_name, event_type, data):
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
        "sender": {
            "user": "tester",
            "service": "test-publish",
            "name": "tester"
        }
    }
    return message

def create_consume_message(queue_name):
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
    message = {
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
    return message

def send_message(sock, message):
    try:
        message_json = json.dumps(message)
        sock.sendall(message_json.encode('utf-8'))
        #print(f"Sent: {message_json}")
        received_data = sock.recv(1024)
        #print(f"Received: {received_data.decode('utf-8')}")
    except Exception as e:
        print(f"An error occurred: {e}")

import hashlib

def generate_hash(input_string):
    # Create a new sha256 hash object
    hash_object = hashlib.sha256()
    # Update the hash object with the bytes of the input string
    hash_object.update(input_string.encode('utf-8'))
    # Get the hexadecimal representation of the hash
    hash_string = hash_object.hexdigest()
    return hash_string 

def receive_message(host, port,):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((host, port))
        print(f"Connected to {host}:{port}")
        while True:
            #time.sleep(1)
            message_json = json.dumps(create_consume_message("test_queue"))
            sock.sendall(message_json.encode('utf-8'))
            received_data = sock.recv(1024)
            if received_data.decode('utf-8') != "No message available":
                try:
                    received_dict = json.loads(received_data.decode('utf-8'))
                    if 'data' in received_dict.get('payload', {}):
                        data = received_dict['payload']['data']
                        print(f"Data: {data}")
                        data["count"] = str(int(data["count"]) + 1)
                        #print(f"Updated Data: {data}")
                        # Send the updated message back to the queue
                        updated_message = create_publish_message(
                            received_dict['queue_name'],
                            received_dict['payload']['event_type'],
                            data
                        )
                        send_message(sock, updated_message)
                except json.JSONDecodeError:
                    print("Received invalid JSON data")

if __name__ == '__main__':
    HOST, PORT = 'localhost', 5672
    
    # Send a message to the queue
    MESSAGE = create_publish_message(
        "test_queue",
        event_type="test_event",
        data={"count": "1"}
    )
    # Establish a single connection
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((HOST, PORT))
        send_message(sock, MESSAGE)
        # Listen to the queue and process messages

        receive_message(HOST, PORT)