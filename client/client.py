import socket
import sys
import threading
import json

table_amount = 100
item_amount = 20
num_thread = 10



def send_recv(socket, msg):
    socket.send(msg)
    _ = socket.recv(512)

def send_recv_json(socket, msg):
    socket.send(msg)
    res = ""
    data = socket.recv(512)
    res += data.decode("utf-8")
    flag = len(data) == 512
    while flag:
        data = socket.recv(512)
        res += data.decode("utf-8")
        flag = len(data) == 512  

    return json.loads(res)


def run_client_add(host, port, thread_id):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        s.connect((host, port))
        print('Connect to %s:%d' % (host, port))
    except:
        print('Unable to connect %s:%d' % (host, port))
        exit(1)

    for table_id in range(0, table_amount):
        item_id_start = item_amount * thread_id
        item_id_end = item_amount * (thread_id + 1)

        for item_id in range(item_id_start, item_id_end):
            send_recv(s, "POST /add/{}/{}".format(table_id, item_id).encode())

    s.close()

def run_client_check_all(host, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        s.connect((host, port))
        print('Connect to %s:%d' % (host, port))
    except:
        print('Unable to connect %s:%d' % (host, port))
        exit(1)

    print("=== Checking ===")

    for table_id in range(0, table_amount):
        response = send_recv_json(s, "GET /query/{}".format(table_id).encode())
        if(len(response) != item_amount * num_thread):
            print("table {} has incorrect amount of items".format(table_id))
            exit(1)

    print("All table has correct amount of items")

    s.close()


if __name__ == '__main__':
    host = '127.0.0.1'
    port = 8080

    if len(sys.argv) == 3:
        host = sys.argv[1]
        port = sys.argv[2]
    elif len(sys.argv) == 1:
        pass
    else:
        print('run: client.py [host] [port]')
        exit(1)

    threads = []

    print("Running {} threads...".format(num_thread))
    print("Each thread adds {} items for each {} tables.".format(item_amount, table_amount))
    for i in range(0, num_thread):
        t = threading.Thread(target=run_client_add, args=(host, port, i))
        threads.append(t)
        t.start()

    for i in range(0, num_thread):
        t = threads[i]
        t.join()

    run_client_check_all(host, port)
