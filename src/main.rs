// ─── Paso 1: Scaffold — LED Blink en ESP32-C3 ───
//
// Este es el "Hello World" de embedded: un blink que verifica que toda la
// cadena funciona: Rust → cross-compile → ESP-IDF → flash → hardware.

// ─── Imports ───

// FreeRtos::delay_ms() — delay cooperativo que cede CPU al scheduler de FreeRTOS.
// NO es busy-wait: el task se suspende y otros tasks pueden correr.
use esp_idf_hal::delay::FreeRtos;

// PinDriver — driver genérico para GPIO. El tipo (Output/Input) se fija
// en compile time: si intentás hacer .read() en un output pin, no compila.
use esp_idf_hal::gpio::PinDriver;

// Peripherals — singleton del hardware. Solo puede crearse UNA vez.
// Garantiza acceso exclusivo a cada periférico via ownership de Rust.
use esp_idf_hal::peripherals::Peripherals;

// Import por side-effect: activa los bindings FFI de esp-idf-sys
// sin traer símbolos al scope. Necesario para que link_patches() funcione.
#[allow(unused_imports)]
use esp_idf_svc::sys as _;

// Facade de logging — define info!(), error!(), warn!(), etc.
// El backend (EspLogger) decide a dónde van los logs (UART/serial).
use log::info;

// ─── Punto de entrada del firmware ───
//
// Retorna Result para poder usar el operador ? durante la inicialización.
// En la práctica main() NUNCA retorna: si lo hace, FreeRTOS destruye el
// task y el watchdog reinicia el chip.
fn main() -> anyhow::Result<()> {
    // ─── Inicialización del sistema (DEBE ser lo primero) ───

    // Resuelve símbolos FFI que el linker necesita pero no están expuestos
    // por defecto en esp-idf-sys. Sin esto: crashes por símbolos faltantes.
    esp_idf_svc::sys::link_patches();

    // Registra EspLogger como backend global de la facade `log`.
    // Después de esto, info!() / error!() envían logs por UART (serial).
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("paso-01-scaffold");

    // ─── Configuración del hardware ───

    // take() usa un AtomicBool interno — si se llama dos veces, retorna Err.
    // Esto garantiza un solo dueño de todo el hardware del chip.
    let peripherals = Peripherals::take()?;

    // gpio8 se MUEVE al PinDriver — ya no existe en peripherals.
    // Si intentás usar peripherals.pins.gpio8 de nuevo, no compila.
    // `mut` es necesario porque set_high()/set_low() mutan el estado del pin.
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;

    info!("LED configurado en GPIO8");

    // ─── Loop principal (nunca termina) ───
    //
    // En embedded, main() corre en un loop infinito.
    // FreeRtos::delay_ms() cede CPU al scheduler (cooperative delay).
    // Sin delay, el watchdog no se alimenta y reinicia el chip.
    loop {
        led.set_high()?;
        info!("LED ON");
        FreeRtos::delay_ms(500);

        led.set_low()?;
        info!("LED OFF");
        FreeRtos::delay_ms(500);
    }
}
