import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { typeOrmConfig } from './db/typeorm.config';
import { EventsModule } from './modules/events/events.module';
import { UsersModule } from './modules/users/users.module';

@Module({
  imports: [TypeOrmModule.forRoot(typeOrmConfig), EventsModule, UsersModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule { }
