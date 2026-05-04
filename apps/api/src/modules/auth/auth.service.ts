import { ConflictException, Injectable, UnauthorizedException } from "@nestjs/common";
import { JwtService } from "@nestjs/jwt";
import * as bcrypt from "bcryptjs";
import { UsersService } from "../users/users.service";
import { AuthResponseDto } from "./dto/auth-response.dto";
import { LoginDto } from "./dto/login.dto";
import { RegisterDto } from "./dto/register.dto";
import { User } from "../users/entities/user.entity";

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly jwtService: JwtService,
  ) {}

  private async issueTokens(user: { id: string; email: string }): Promise<{
    accessToken: string;
    refreshToken: string;
  }> {
    const payload = { sub: user.id, email: user.email };
    const accessExpiresIn = "15m";
    const refreshExpiresIn = "30d";

    const accessToken = await this.jwtService.signAsync(payload, {
      expiresIn: accessExpiresIn,
      secret: process.env.JWT_ACCESS_SECRET ?? "dev_access_secret",
    });

    const refreshToken = await this.jwtService.signAsync(payload, {
      expiresIn: refreshExpiresIn,
      secret: process.env.JWT_REFRESH_SECRET ?? "dev_refresh_secret",
    });

    return { accessToken, refreshToken };
  }

  private toAuthResponse(
    user: User,
    tokens: { accessToken: string; refreshToken: string },
  ): AuthResponseDto {
    return {
      ...tokens,
      user: {
        id: user.id,
        email: user.email,
        name: user.name,
      },
    };
  }

  async register(dto: RegisterDto): Promise<AuthResponseDto> {
    const existing = await this.usersService.findByEmail(dto.email);
    if (existing) {
      console.log("test");
      throw new ConflictException("Email already in use");
    }

    const password = await bcrypt.hash(dto.password, 10);
    const user = await this.usersService.create({
      email: dto.email,
      name: dto.name,
      password,
    });

    const tokens = await this.issueTokens(user);
    return this.toAuthResponse(user, tokens);
  }

  async login(dto: LoginDto): Promise<AuthResponseDto> {
    const user = await this.usersService.findByEmailWithPassword(dto.email);
    if (!user?.password) {
      throw new UnauthorizedException("Invalid credentials");
    }

    const ok = await bcrypt.compare(dto.password, user.password);
    if (!ok) {
      throw new UnauthorizedException("Invalid credentials");
    }

    const tokens = await this.issueTokens(user);
    return this.toAuthResponse(user, tokens);
  }

  // async refresh(dto: RefreshDto): Promise<AuthResponseDto> {
  //   let payload: any;
  //   try {
  //     payload = await this.jwtService.verifyAsync(dto.refreshToken, {
  //       secret: process.env.JWT_REFRESH_SECRET ?? "dev_refresh_secret",
  //     });
  //   } catch {
  //     throw new UnauthorizedException("Invalid refresh token");
  //   }

  //   const user = await this.usersService.findByEmail(payload.email);
  //   if (!user) {
  //     throw new UnauthorizedException("Invalid refresh token");
  //   }

  //   const tokens = await this.issueTokens(user);
  //   return this.toAuthResponse(user, tokens);
  // }

  async logout(userId: string): Promise<void> {
    void userId;
  }
}
