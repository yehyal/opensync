import { Device } from "src/modules/devices/entities/device.entity";
import { Event } from "src/modules/events/entities/event.entity";
import {
  Column,
  CreateDateColumn,
  Entity,
  Index,
  OneToMany,
  PrimaryGeneratedColumn,
  UpdateDateColumn,
} from "typeorm";

@Entity({ name: "users" })
export class User {
  @PrimaryGeneratedColumn("uuid")
  id: string;

  @Column({ type: "text" })
  name: string;

  @Index()
  @Column({ type: "text", unique: true })
  email: string;

  @Column({ type: "text" })
  password: string;

  @CreateDateColumn({ type: "timestamptz" })
  createdAt: Date;

  @UpdateDateColumn({ type: "timestamptz" })
  updatedAt: Date;

  @OneToMany(() => Device, (device) => device.user)
  devices: Device[];

  @OneToMany(() => Event, (event) => event.user)
  events: Event[];
}
