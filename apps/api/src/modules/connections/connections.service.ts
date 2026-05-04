import { Injectable } from "@nestjs/common";
import { Socket } from "socket.io";

@Injectable()
export class ConnectionsService {
  private connections: Map<string, Map<string, Socket>>;

  constructor() {
    this.connections = new Map();
  }

  add(userId: string, deviceId: string, socket: Socket) {
    if (!this.connections.has(userId)) {
      this.connections.set(userId, new Map<string, Socket>([[deviceId, socket]]));
    } else {
      this.connections.get(userId).set(deviceId, socket);
    }
  }

  getConnections(userId: string): Map<string, Socket> | undefined {
    return this.connections.get(userId);
  }

  delete(userId: string, deviceId: string) {
    if (this.connections.has(userId)) {
      this.connections.get(userId).delete(deviceId);
      if (this.connections.get(userId).size === 0) {
        this.connections.delete(userId);
      }
    }
  }
}
