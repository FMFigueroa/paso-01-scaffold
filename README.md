# Rust Embedded desde Cero

## paso-01-scaffold

[![ESP32 CI](https://github.com/FMFigueroa/paso-01-scaffold/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/FMFigueroa/paso-01-scaffold/actions/workflows/rust_ci.yml)

<p align="center">
  <img src="docs/rust-board.png" alt="ESP32-C3-DevKit-RUST-1" width="600">
</p>

Scaffold base para ESP32-C3 en Rust. LED blink en GPIO8 que verifica que toda la cadena funciona: Rust → cross-compile → ESP-IDF → flash → hardware.

## Pre-Requisitos

```bash
rustup --version          # Rust (nightly)
cmake --version           # Build system para ESP-IDF
ninja --version           # Backend de compilación
espflash --version        # Herramienta de flash
which ldproxy             # Linker proxy
```

Si falta algo:

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dependencias (macOS)
brew install cmake ninja python3

# Target RISC-V
rustup target add riscv32imc-unknown-none-elf

# Herramientas de flash y linkeo
cargo install espflash cargo-espflash ldproxy
```

## Compilar y flashear

```bash
cargo espflash flash --release --monitor
```

El primer build descarga y compila el ESP-IDF SDK (~2GB). Los siguientes son incrementales.

## Estructura

```
.cargo/config.toml      # Cross-compilation para riscv32imc-esp-espidf
Cargo.toml              # Dependencias: esp-idf-hal, esp-idf-svc, anyhow, log
rust-toolchain.toml     # Nightly + rust-src (build-std)
build.rs                # Integración con ESP-IDF via embuild
sdkconfig.defaults      # FreeRTOS: 8KB stack, 1000Hz tick
src/main.rs             # LED blink en GPIO8 (500ms on/off)
```

## Dependencias

| Crate         | Uso                                               |
| ------------- | ------------------------------------------------- |
| `esp-idf-hal` | Hardware Abstraction Layer (GPIO, SPI, I2C, PWM)  |
| `esp-idf-svc` | Servicios de alto nivel (WiFi, HTTP, NVS, logger) |
| `log`         | Facade de logging (info!, error!, warn!)          |
| `anyhow`      | Manejo de errores con contexto                    |

## Documentacion

Te invito a unirte a nuestro servidor de Discord para que no te pierdas el desarrollo completo del curso **Rust - Embedded desde Cero**. Encontraras documentacion detallada de cada paso, explicaciones profundas de conceptos, cuestionarios y soporte directo.

<a href="https://discord.gg/dYrqe9HZfz">
  <img alt="Discord" width="35px" src="https://img.icons8.com/external-justicon-lineal-color-justicon/64/external-discord-social-media-justicon-lineal-color-justicon.png"/>
</a>&ensp;
<a href="https://discord.gg/dYrqe9HZfz"><strong>Unirse al servidor — Curso Rust Embedded</strong></a>

## Roadmap

> Este repo es el **Paso 1** del curso **Rust Embedded desde Cero**.

- **[Paso 1 — Scaffold del proyecto](https://github.com/FMFigueroa/paso-01-scaffold)** ← _este repo_
- [Paso 2 — WiFi Station](https://github.com/FMFigueroa/paso-02-wifi-station)
- Paso 3 — LED PWM
- Paso 4 — WebSocket Client
- Paso 5 — Light State Management
- Paso 6 — Telemetria
- Paso 7 — Time Sync (SNTP)
- Paso 8 — Schedule & Auto Mode
- Paso 9 — Concurrencia & Watchdog

## Licencia

[MIT](LICENSE)
