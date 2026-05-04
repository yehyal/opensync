import { Injectable } from "@nestjs/common";
import {
  ConnectedSocket,
  MessageBody,
  OnGatewayConnection,
  OnGatewayDisconnect,
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from "@nestjs/websockets";
import { Server, Socket } from "socket.io";
import { ConnectionsService } from "../connections/connections.service";
import { JwtService } from "@nestjs/jwt";

@Injectable()
@WebSocketGateway({
  cors: {
    origin: "*",
  },
})
export class EventsGateway implements OnGatewayConnection, OnGatewayDisconnect {
  @WebSocketServer()
  server: Server;

  constructor(
    private readonly connectionsManager: ConnectionsService,
    private readonly jwtService: JwtService,
  ) {}

  broadcast(text: string): void {
    this.server.emit("event.created", text);
    this.server.emit("event", text);
  }

  @SubscribeMessage("event")
  handleCreateEvent(@MessageBody() data: string, @ConnectedSocket() socket: Socket) {
    const clientId = socket.id;
    console.log(clientId, data);
    this.broadcast(`${data} + ${clientId} test`);
  }

  handleConnection(socket: Socket): void {
    try {
      const token =
        (socket.handshake?.query?.token as string) ||
        (socket.handshake?.headers?.authorization as string)?.split(" ")[1];

      if (!token) {
        socket.disconnect();
        return;
      }

      const payload = this.jwtService.verify(token);
      const userId = payload.sub || payload.id || payload.userId;
      const deviceId = (socket.handshake?.query?.deviceId as string) || socket.id;

      if (!userId) {
        socket.disconnect();
        return;
      }

      socket.data.userId = userId;
      socket.data.deviceId = deviceId;

      this.connectionsManager.add(userId, deviceId, socket);
      console.log(`Client connected: userId=${userId}, deviceId=${deviceId}`);
    } catch (error) {
      console.error("Connection authentication failed:", error.message);
      socket.disconnect();
    }
  }

  handleDisconnect(socket: Socket): void {
    const userId = socket.data?.userId;
    const deviceId = socket.data?.deviceId || socket.id;

    if (userId) {
      this.connectionsManager.delete(userId, deviceId);
      console.log(`Client disconnected: userId=${userId}, deviceId=${deviceId}`);
    }
  }
}
