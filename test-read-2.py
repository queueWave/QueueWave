import socket

def receive_message(host, port):
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.connect((host, port))
            print(f"Connected to {host}:{port}")
            while True:
                received_data = sock.recv(1024)
                if not received_data:
                    break
                print(f"Received: {received_data.decode('utf-8')}")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == '__main__':
    HOST, PORT = 'localhost', 5672
    receive_message(HOST, PORT)