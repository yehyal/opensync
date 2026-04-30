import { Injectable, NotFoundException } from "@nestjs/common";
import { CreateDeviceDto } from "./dto/create-device.dto";
import { UpdateDeviceDto } from "./dto/update-device.dto";
import { InjectRepository } from "@nestjs/typeorm";
import { Device } from "./entities/device.entity";
import { Repository } from "typeorm";

@Injectable()
export class DevicesService {
  constructor(
    @InjectRepository(Device)
    private readonly deviceRepo: Repository<Device>,
  ) {}

  async create(createDeviceDto: CreateDeviceDto): Promise<Device> {
    const device = this.deviceRepo.create(createDeviceDto);
    return this.deviceRepo.save(device);
  }

  async findAll(userId: string): Promise<Device[]> {
    return this.deviceRepo.find({ where: { userId } });
  }

  async findOne(id: string): Promise<Device> {
    const device = this.deviceRepo.findOne({ where: { deviceId: id } });
    if (!device) {
      throw new NotFoundException(`Device with id: ${id} not found`);
    }
    return device;
  }

  async update(id: string, updateDeviceDto: UpdateDeviceDto): Promise<Device> {
    const device = await this.findOne(id);
    const updated = this.deviceRepo.merge(device, updateDeviceDto);
    return this.deviceRepo.save(updated);
  }

  async remove(id: string): Promise<void> {
    const device = await this.findOne(id);
    await this.deviceRepo.remove(device);
  }
}
