import cv2
import msgpack
import socket
img = cv2.imread("scream.jpg")
image_packed = msgpack.packb(img.tolist())
sock = socket.socket(socket.AF_INET, # Internet
                        socket.SOCK_STREAM) # UDP
sock.connect(("0.0.0.0", 12345))
sock.send(image_packed)
