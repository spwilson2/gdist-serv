=====
Slimy
=====

Slimy is a distributed command server.
In its most simple form, slimy acts as a server to schedule commands on a collection of clients.

Slimy needs to are tradeoffs in order to accomplish goals.
As a priority the following are prioritized above all else:

---
**Speed of execution**

Scheduled commands should be schedueld and executed quickly with little latency.
As Slimy is designed to be a distributed build scheduler, it will likely received a constant stream of compile commands, and therefore should delay the execution of commands as little as possible.

---
**Maintainable code**

Slimy is being built as a side project with the intent that is be used at GHS as a build scheduler.
Code will eventually be put into production, and will likely be used and possibly go untouched for a long period of time.
This means code should be easy to both understand and modify without deep system knowledge.




Server - Client Architecture
============================

The server-client connections is split into two layers in order to enable multiple methods of communication between clients and servers.


Connection layer
----------------

The lower layer is the Connection layer.
This can best be understood as a data stream available for use between two hosts.
This could be implemented as a TCP channel, ad-hoc UDP solution, or even a shared NFS file.
In our simple implementation, we opt to use an ad-hoc UDP based channel.


Connection API::

    Future<void> SendAsync(data) - Attempt to send the packet (order not garunteed), future is returned which can be checked for exception.
    Future<data> RecvAsync() - Attempt to receive a packet, a future is returned which will either contain the data or raise a DroppedConnection exception.
    Future<void> OnceClosed() - Return a future that can be used to await closure
    Close() - Close the connection, any Send/Recv commands will be canceled.
    Bool Closed() - Return if the connection is still valid (Note: a non-closed connection is not garunteed to be able to send/recv on)


We also expect Connection implentations to be created via a ConnectionFactory:

Connection Factory API::

    Future<Connection> ConnectAsync()


Agent layer
-----------

The next layer is the Agent layer.
This layer adds unique identification to connections.
It adds the ability to maintain identification after connection reconnects.
This also abstracts away the lower connection protocol, so the agent layer can be used with any connection implementation.

Agent API::
    Future<void> SendAsync(data) - Attempt to send the data (order not garunteed), future is returned which can be checked for exception.
    Future<data> RecvAsync() - Attempt to receive data, a future is returned which will either contain the data or raise a DroppedConnection exception.

    Bool - Unreachable() - Return if the Agent is currently unreachable (there is no backing active connection)

Properties::
    - UID


UDP Connection Implementation
-----------------------------

The UDP Connection Factory will split connection initialization by client and server attributes.

FactoryAPI::

    UDPConnectionFactory(client_not_server)

Limitations
~~~~~~~~~~~

Because this UDP implementation does not implement any "sliding window" component, message sizes will be limited to the max UDP packet size.

Messages
~~~~~~~~

All messages will contan a magic header with:
- Version - 0x00010000 - 4B
- Magic - 0x4748530000000000 - 8B

- RegisterConnectionRequest
- RegisterConnectionResponse
    - (Newly created) Connection UID
- RegisterConnectionResponseAck
    - Connection UID
- HeartbeatPing
    - Connection UID
- DatagramSend
    - Connection UID
    - Datagram UID
    - Data
- DatagramAck
    - Connection UID
    - Datagram UID
    - Data
- DisconnectRequest
    - Connection UID
- Disconnected
    - Connection UID

Connection Process
~~~~~~~~~~~~~~~~~~

Connecting to the server

1. Client sends a RegisterConnectionRequest message.
2. Server creates a new UID for the connection, responds to client with RegisterConnectionResponse.
3. Client responds with RegisterConnectionResponseAck, Server keeps sending response until ack is received up to 5 times.
4. Client and server will now send a heartbeat message to each other every half second to maintain the connection.

If a client doesn't receive a heartbeat for 2 seconds, the client assumes the connection is dropped and will attempt the create a new connection.
If a client doesn't receive a heartbeat for 2 seconds, the server assumes the connection is dropped and will notify client handle users.

---

Sending a message

1. API SendAsync is called on either end of the connection.
2. Sender transmits via DatagramSend
3. Receiver receives message and responds with DatagramAck

If the sender does not receive a DatagramAck, exponential backoff sending DatagramSend up to 5 times.

---

Disconnecting

Same method as sending a message, but using DisconnectRequest and Disconnected.
Once a first Disconnected is sent, the sender will report the connection as closed.

---

Unexpected Packets

A UDP user can receive different packets at unexpected times.
In the case that a UDP client/server receives a packet which contains an unrecognized UID, a Disconnected message will be sent in response with that UID.


Agent Layer Implementation
--------------------------

The Agent Layer sits on top of the connection layer.
This implementation of an agent will maintain only a single connection per Agent.
The Agent will manage creation of connections in order to maintain communication between hosts.

Messages
~~~~~~~~

- Datagram - Send data between agents
    - Data

---

Lifecycle of an Agent

1. Agent is created Agent(ConnectionFactory)
2. Agent asynchronously attempts to establish a single connection by calling ConnectionFactory.ConnectAsync() and awaiting.
   (This decouples the agent from the connection and enables us to use any connection implementation)

User now calls SendAsync/RecvAsync

3. Data is transferred via the connection, the call is forwarded to same connection method.

If a send or recv fails with a ConnectionDisconnected exception, the Agent will re-create the connection.


Work Scheduler
==============

Work Tree
---------

Slimy organizes work which can be distributed across nodes into a Work Tree.
This tree is formed by a DAG of Tasks and their dependendant tasks.
That is the root of the tree depends on the completion of all tasks.
A leaf node in the tree is a task which has no dependencies (and could be started right away).

Work Distributing
-----------------

Currently, Slimy will only send a single task from the Work Tree of the Manger Node to a Worker Node at a time.
The downside of this is that communicaton between the Manger Node and the Worker Node will be on the critical path for each the completion of every task.
The upside is that this makes the scheduler much simpler as there is no need to think about distributed work stealing algorithms.

Scheduler API
-------------
::

    designed

Manager - Worker Architecture
==============================

With connection logic properly encapuslated by the `Server - Client Architecture`, we now can split workers and managers into their own distinct implementations.

Manager Node
------------

In a cluster there is a single Manager node.
The Manager's role is to schedule and forward Task Queues to connected worker nodes.

While Workers are completing tasks, they will continually push task results to the Manger node.
If a task fails to complete the Manager can revoke Workers' tasks.
(This is useful in the case of using Slimy as a distributed build system.
If compilation fails, there's no reason to continue the build because not all object files required to link will be created.)

.. Comment
    Note that we wrote "Task Queues".
    We might be able to implement some sort of Cilk-like scheduler where we hand part of the DAG to a worker node and then it's the worker node's job to finish that DAG.


API::

    _SendTaskTree(worker, tree)

Worker Node
-----------

In a cluster there are one or more Worker nodes.
Worker nodes receive Task Queues from the manager node.
Worker nodes begin execution of tasks from this queue of tasks.
As tasks are completed, results are reported back to the Manager node.


.. Comment
    Lifecycle Planning
    ==================
    - Server starts up
    - Server receives a list of jobs
    - Server schedules jobs on the pool of workers
    Async Chain:
    - Server listens for connections

API::

    TODO