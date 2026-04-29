import { Body, Controller, HttpCode, Post } from "@nestjs/common";
import { EventsGateway } from "./events.gateway";

@Controller()
export class EventsController {
  constructor(private readonly eventsGateway: EventsGateway) {}

  @Post(["event", "events"])
  @HttpCode(201)
  handleHttpEvent(@Body() event: { text: string }): void {
    // Broadcast to all connected socket.io clients.
    console.log(event);
    this.eventsGateway.broadcast(event.text);
  }
}
