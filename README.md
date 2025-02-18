**Container-Flow** is a Rust library providing a flexible client for interacting with container APIs (such as Docker and Podman). 
It supports both async and sync modes, offering you the flexibility to choose the best approach for your application. 
Regardless of the transport layer—be it HTTP, Unix sockets, or Windows named pipe the API remains consistent, 
so you can seamlessly switch between transport methods without changing how you interact with containers. 
Future updates will include support for SSH transport for remote container management.

Key Features:

 - Unified async and sync client for interacting with container APIs (Docker/Podman).
 - Same API interface across different transport layers (HTTP(S), Unix socket, Windows named pipe, SSH).

**Container-Flow** simplifies communication with containers by providing a consistent and flexible API, 
whether you're working locally or remotely with different transport methods.