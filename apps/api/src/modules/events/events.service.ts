import { Injectable } from "@nestjs/common";
import { Repository } from "typeorm";
import { Event } from "./entities/event.entity";
import { InjectRepository } from "@nestjs/typeorm";
import { CreateEventDto } from "./dto/create-event.dto";

@Injectable()
export class EventsService {
  constructor(
    @InjectRepository(Event)
    private readonly eventsRepo: Repository<Event>,
  ) {}

  async create(dto: CreateEventDto): Promise<Event> {
    return this.eventsRepo.create(dto);
  }

  async findAllByUserId(userId: string): Promise<Event[]> {
    return this.eventsRepo.find({ where: { userId } });
  }
}
