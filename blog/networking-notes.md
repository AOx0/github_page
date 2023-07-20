# Networking Notes

These notes are based on the book _Network Programming with Rust_ by Abhishek Chanda, the excellent _Guide to Network Programming_ by Brian Hall, and other sources that describe how networking works. My objective here is to have me write down the concepts so I can get a better understanding of them.

## Internet Sockets

On Linux, everything is exposed by the kernel as files, a socket is nothing but a file descriptor to which we can read and write to send and receive messages from other computers or processes from the same machine.

There are different kinds of sockets for various purposes. The resources I read pay special attention to Internet Sockets (see `man -S2 socket`). Internet Sockets have two main variants, connection-centered and connection-less. Examples of these two designs are TCP and UPD protocols, also referred to as Stream Sockets (`SOCK_STREAM`) and Datagram Sockets (`SOCK_DGRAM`), respectively.

### Connection Services

The difference between connection-centered and connectionless services is pretty straightforward from the name, where TCP (and connection-centered services) sends metadata so that peers know who they are talking to, at what point of the conversation they are at, and have client-server-established connections that require an acknowledged communication from both ends. On the other hand, UDP (and connectionless services) provides the basics for sending information to a client with no requirements for a connecting negotiation. Protocols like UDP are less suitable for communications that require correct sequences or even arrivals of messages but, in contrast, can start sending messages much simpler and quicker.

Even though UDP has no built-in sequence or received tracking, it can be easily "extended" to include an `AKC` (acknowledged) system, like the one used for two-way connection negotiations from TCP [2].

## TCP

As we saw, TCP is a connection-centered internet socket protocol. Its design gives TCP the properties that make it suitable for implementing programs like ssh. TCP provides reliable, ordered, and error-checked delivery of a stream of octets (bytes) between applications running on hosts communicating via an IP network. [1]

Abhishek [3] describes the steps TCP performs as follows:

1- The server starts by:

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- acquiring a socket (`man -S2 socket`)

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- binding an IP address to the socket (`man -S2 bind`)

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- listening for messages (`man -S2 listen`)

2- The client calls `connect` on the server. Effectively puts it in a state where it waits for the server response under the `SYN_SENT` name.

3- The client sends an `SYN` packet with a control flag that issues the synchronization of sequence numbers [3]. That is, start the sequence at an agreed number. This sequence number lets the receiver reorder messages.

4- The server calls `accept` on the client request. These steps, starting from the `connect` function call perform the three-way handshake necessary so that the connection can be two-way ended.

5- The server sends an `ACK` + `SYN` to inform the client it acknowledged the request and sends the initial sequence number. As well as the client, the server now enters a waiting-for-response state under the `SYN_RECV` name. It's now waiting for the final client acknowledgment.

6. The server sends the final acknowledgment (`ACK`) packet, and both machines turn to the `ESTABLISHED` state. They can use the `send` and `recv` functions to share data at this state.

### Closing connection

A similar process happens when the connection wants to be closed. The two peers communicate the desire to disconnect along with OK messages to ensure they are on the same page.

TODO

## IP

We use sockets to connect and communicate with something. We specify what that thing is with its address. There are Ethernet addresses, Autonomous system numbers, and IP addresses. 

IP addresses come in two versions: 4 and 6. IPv4 addresses are made from 32 bits, while IPv6 addresses use 128 bits. With this many bits on IPv6, we unlock more possible addresses, which is critical for a world where the number of people connected to the internet is so large. With 32-bit addresses making 2^32 (~ 4 billion) possibilities and 128-bit up to 2^128 (~ a lot).

## Basic echo server C

Let's begin by creating a very simple never ending loop that accepts for connections, reads its messages and returns the exact same message back.

The first step is to create the socket. Although this was difficult in the past, now we can use `getaddrinfo` to quickly and correctly get the necessary information for binding to IPv4/IPv6 addresses. In the following example, we get the information for a socket on the localhost `::` port `9096` and issue the socket instantiation to the OS using the `socket` function.

```c
#include <netdb.h>
#include <sys/socket.h>
#include <stdio.h>

int main() {
  struct addrinfo *info;
  struct addrinfo hints = {
    .ai_flags = AI_PASSIVE,    // Suitable for bind/accept 
    .ai_family = PF_UNSPEC,    // Any of IPv4 or IPv6
    .ai_socktype = SOCK_STREAM // TCP
  };
  
  // Make the system complete the information for localhost port 9096
  // Uses hints to get the correct address information so we can bind to it.
  // We intend to accept both IPv4 and IPv6, hence we bind to IPv6 address [::].
  int getaddrinfo_status = getaddrinfo("::", port, &hints, &info);

  // We handle any error while getting the address information structure.
  if (getaddrinfo_status != 0) {
    fprintf(stderr, "ERROR :: getaddrinfo: %s\n",
            gai_strerror(getaddrinfo_status));
    return 1;
  }

  // Issue the socket with the given info
  int s = socket(info->ai_family, info->ai_socktype, info->ai_protocol);
  if (s == -1) {
    perror("ERROR :: socket");
    freeaddrinfo(info); // Free the addrinfo structure. 
    return 1;
  }

  printf("INFO :: Got socket descriptor %d\n", s);
}
```

Not that we have a socket that can listen, the following step is to start listening for messages by binding the socket descriptor with the desired service address;

```c
/* .. past includes .. */ 

#define MAX_CONNECTIONS 50

int main() {
  /* .. past code .. */

  int reuse_addr = 1;
  // Allow multiple bindings to the given address/port combination on bind
  if (setsockopt(s, SOL_SOCKET, SO_REUSEADDR, &reuse_addr, sizeof(int)) == -1) {
    perror("ERROR :: setsockopt");
    close(s); // Close the server file descriptor
    freeaddrinfo(info); // Free the addrinfo structure. 
    return 1;
  }

  // Bind socket and address/port combination
  int bind_status = bind(s, info->ai_addr, info->ai_addrlen);
  if (bind_status == -1) {
    perror("ERROR :: bind");
    close(s); // Close the server file descriptor
    freeaddrinfo(info); // Free the addrinfo structure. 
    return 1;
  }

  printf("INFO :: Bound socket to port %s\n", port);

  // Start listening for connections.
  int listen_status = listen(s, MAX_CONNECTIONS);
  if (listen_status == -1) {
    perror("ERROR :: listen");
    close(s); // Close the server file descriptor
    freeaddrinfo(info); // Free the addrinfo structure. 
    return 1;
  }

  puts("INFO :: Server now listening");
}
```

Now that the server is running and listening, it's time to accept connections and handle its messages. For this reason, we will create an infinite loop that issues new threads for handling any accepted connections.

```c
/* .. past includes .. */
#include <pthread.h>
#include <stdlib.h>

struct conh_args {
  int s;
  int cs;
  socklen_t client_len;
  struct sockaddr_storage client_addr;
};

void *handle_conn(void *targs) {
  /* TODO */
  
  free(targs); // Clean args from heap
  pthread_exit(NULL); // Terminate thread
}

int main() {

  while (1) {
    // Accept the new connection. Returns a new file descriptor.
    int cs = accept(s, (struct sockaddr *)&client_addr, &client_len);

    // If the connection descriptor is not an error, then spawn a new thread
    if (cs > 0) {
      // We copy the necessary arguments to a struct in the heap so we can
      // safely move its ownership to the new thread.
      struct conh_args *args, targs = {.s = s,
                                       .cs = cs,
                                       .client_len = client_len,
                                       .client_addr = client_addr};
      args = malloc(sizeof(struct conh_args));
      *args = targs;

      // We spawn the new thread and give it ownership of the contents of
      // args;
      (void)pthread_create(&(ids[i]), NULL, &handle_conn, (void *)args);
      i++;
    }
  }

  return 0;
}
```


On Linux, the `accept` method returns a new descriptor for us to maintain that single connection. We could perform a loop to accept/send messages in a REPL style, but for the sake of simplicity, we are just going to read whatever the user sends, send it back to them, close the connection, and exit the thread.

```c
/* includes */
#include <string.h>

void *handle_conn(void *targs) {
  struct conh_args *args = (struct conh_args *)targs;
  char hoststr[NI_MAXHOST] = {0};
  char portstr[NI_MAXSERV] = {0};

  // Get port and ip strings
  int rc = getnameinfo((struct sockaddr *)&args->client_addr, args->client_len,
                       hoststr, sizeof(hoststr), portstr, sizeof(portstr),
                       NI_NUMERICHOST | NI_NUMERICSERV);

  // Show the address that the socket handles
  if (rc == 0)
    printf("INFO :: Accepting connection from %s %s\n", hoststr, portstr);
  else
    printf("INFO :: Accepting connection\n");

  while (1) {
    char msg[500] = "\0";
    // Read a max of 500 bytes from the client
    recv(args->cs, (void *)&msg, 500, 0);
    // Send the same message through the socket
    send(args->cs, (void *)msg, strlen(msg), 0);
  }

  // Close the socket
  shutdown(args->cs, 2);

  free(targs);        // Clean args from heap
  pthread_exit(NULL); // Terminate thread
}
```

## Now goes Rust


# Refs

[1] https://en.wikipedia.org/wiki/Transmission_Control_Protocol

[2] https://beej.us/guide/bgnet/html/#what-is-a-socket

[3] Abhishek Chanda (). _Network Programming with Rust_. 
