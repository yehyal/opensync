import { Body, Controller, HttpCode, Post, UseGuards } from "@nestjs/common";
import { EventsGateway } from "./events.gateway";
import { JwtAuthGuard } from "../auth/jwt-auth.guard";

@Controller()
export class EventsController {
  constructor(private readonly eventsGateway: EventsGateway) {}

  @UseGuards(JwtAuthGuard)
  @Post("events")
  @HttpCode(201)
  handleHttpEvent(@Body() event: { text: string }): void {
    // Broadcast to all connected socket.io clients.
    console.log(event);
    this.eventsGateway.broadcast(event.text);
  }
}
