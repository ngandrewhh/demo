### Introduction
A basic text editor based on crossterm, aiming to create a shared editing function through single server-multiple client setup through TCP.
### Functionality
- Completed active logging of insertion/deletion log to send over to other clients for replication.
- Completed basic server-client structure.
### To-dos
- Find an efficient way to broadcast changes batched through server.
- Test thoroughly packet losses/connection interruptions.
- `[Low priority]` Other 'basic' text editor functionality such as scrolling, better screen repainting, text coloring.
- `[Low priority]` Unicode support.