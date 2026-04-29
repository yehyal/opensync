import {
  ConnectedSocket,
  MessageBody,
  OnGatewayConnection,
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from "@nestjs/websockets";
import { Server, Socket } from "socket.io";

@WebSocketGateway({
  cors: {
    origin: "*",
  },
})
export class EventsGateway implements OnGatewayConnection {
  private readonly connectedClients: Map<string, Socket> = new Map();

  @WebSocketServer()
  server: Server;

  broadcast(text: string): void {
    // Emit both for compatibility; prefer `event.created`.
    this.server.emit("event.created", text);
    this.server.emit("event", text);
  }

  @SubscribeMessage("event")
  handleCreateEvent(@MessageBody() data: string, @ConnectedSocket() socket: Socket) {
    const clientId = socket.id;
    console.log(clientId, data);

    // Send to everyone (including the sender) so clients don't miss events.
    this.broadcast(`${data} + ${clientId} test`);
  }

  handleConnection(socket: Socket): void {
    const clientId = socket.id;
    this.connectedClients.set(clientId, socket);
    console.log(`Client connected: ${clientId}`);
    socket.on("disconnect", () => {
      this.connectedClients.delete(clientId);
    });
  }
}
