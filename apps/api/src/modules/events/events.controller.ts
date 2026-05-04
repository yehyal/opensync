import { Body, Controller, HttpCode, Post, Req, UseGuards } from "@nestjs/common";
import { JwtAuthGuard } from "../auth/jwt-auth.guard";
import { CreateEventDto } from "./dto/create-event.dto";
import { EventsService } from "./events.service";
import { Request } from "express";
import { User } from "../users/entities/user.entity";

@Controller()
export class EventsController {
  constructor(private readonly eventsService: EventsService) {}

  @UseGuards(JwtAuthGuard)
  @Post("events")
  @HttpCode(201)
  async handleHttpEvent(
    @Body() createEventDto: CreateEventDto,
    @Req() req: Request & { user: User },
  ): Promise<void> {
    await this.eventsService.create(req.user.id, createEventDto);
  }
}
