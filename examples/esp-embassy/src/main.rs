#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::RefCell;

use critical_section::Mutex;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output},
    timer::timg::TimerGroup,
};
use log::info;
use pin_logger::pin_log;
use pin_logger::{PinLogger, SetPin};
use static_cell::StaticCell;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let pins = [Output::new(
        peripherals.GPIO25,
        Level::Low,
        Default::default(),
    )];
    let pins = pins.map(|pin| {
        static PIN_CELL: StaticCell<Output> = StaticCell::new();
        PIN_CELL.init(pin) as &mut dyn SetPin
    });

    pin_logger::load_names!(NAMES, NAMES_LEN);
    static PINS_CELL: StaticCell<[&mut dyn SetPin; pin_logger::no_pins(NAMES.len())]> =
        StaticCell::new();
    let pins = PINS_CELL.init(pins);

    static MUTEX_PIN_LOGGER: Mutex<RefCell<Option<PinLogger>>> = Mutex::new(RefCell::new(None));
    critical_section::with(|cs| {
        MUTEX_PIN_LOGGER
            .borrow(cs)
            .replace(Some(pin_logger::init2!(pins)));
    });

    spawner.spawn(task().unwrap());

    loop {
        critical_section::with(|cs| {
            let mut foo = MUTEX_PIN_LOGGER.borrow(cs).borrow_mut();
            let l = foo.as_mut().unwrap();
            pin_log!(l, "Hello from the main loop");
        });
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn task() {
    loop {
        info!("Hello from the task!");
        Timer::after(Duration::from_millis(700)).await;
    }
}
