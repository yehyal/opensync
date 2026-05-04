import { Injectable } from "@nestjs/common";
import { Repository } from "typeorm";
import { Event } from "./entities/event.entity";
import { InjectRepository } from "@nestjs/typeorm";
import { CreateEventDto } from "./dto/create-event.dto";
import { ConnectionsService } from "../connections/connections.service";
import { EventsGateway } from "./events.gateway";

@Injectable()
export class EventsService {
  constructor(
    @InjectRepository(Event)
    private readonly eventsRepo: Repository<Event>,
    private readonly connectionsManager: ConnectionsService,
    private readonly eventsGateway: EventsGateway,
  ) {}

  async create(userId: string, dto: CreateEventDto): Promise<Event> {
    const event = this.eventsRepo.create({
      userId,
      deviceId: dto.deviceId,
      content: dto.content,
      hash: dto.hash,
      timestamp: new Date(),
    });
    await this.eventsRepo.save(event);

    const connectedDevices = this.connectionsManager.getConnections(userId);
    if (connectedDevices) {
      connectedDevices.forEach((socket, deviceId) => {
        if (!socket) return;
        socket.emit("event.created", event);
        console.log(`Socket event emitted for device: ${deviceId} and user: ${userId}`);
      });
    }

    return event;
  }

  async findAllByUserId(userId: string): Promise<Event[]> {
    return this.eventsRepo.find({ where: { userId } });
  }
}
