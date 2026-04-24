import { Body, Controller, Get, HttpCode, Post } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) { }

  @Get("test")
  getHello(): string {
    return this.appService.getHello();
  }
  @Post("event")
  @HttpCode(201)
  handleEvent(@Body() event: { text: string }): void {
    console.log(event);
    console.log("EVENT");
    return
  }
}
