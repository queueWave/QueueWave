import socket

def send_message(host, port, message):
    # Opret en TCP/IP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    try:
        # Forbind til serveren
        sock.connect((host, port))
        print(f"Connected to {host}:{port}")

        # Send beskeden
        sock.sendall(message.encode('utf-8'))
        print(f"Sent: {message}")

        # Modtag respons
        response = sock.recv(1024)  # Juster bufferstørrelsen efter behov
        print(f"Received: {response.decode('utf-8')}")

    finally:
        # Luk forbindelsen
        sock.close()
        print("Socket closed.")

if __name__ == '__main__':
    HOST, PORT = 'localhost', 5672  # AMQP standardport, ændr efter behov
    MESSAGE = "Hello, AMQ Server!\n"  # Husk newline, hvis din server bruger det som beskedseparator
    for i in range(10):
        send_message(HOST, PORT, MESSAGE)
