import { Module } from "@nestjs/common";
import { EventsController } from "./events.controller";
import { EventsGateway } from "./events.gateway";
import { TypeOrmModule } from "@nestjs/typeorm";
import { Event } from "./entities/event.entity";
import { EventsService } from "./events.service";

@Module({
  imports: [TypeOrmModule.forFeature([Event])],
  controllers: [EventsController],
  providers: [EventsGateway, EventsService],
  exports: [EventsService],
})
export class EventsModule {}
