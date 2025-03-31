#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::{panic::PanicInfo};
use mik32v2_pac::{Peripherals};
use mik32_rt::entry;
use riscv::{self as _};

fn scale(percent: u32) -> u32 {
    assert!(percent <= 100);
    return 40 * percent + 4000
}

fn timer_init(p: &Peripherals) {
    
    p.pm.clk_apb_p_set().modify({|_, w| w
        .timer32_1().enable()
    });

    p.pad_config.pad0_cfg().modify(|_, w| w.port_0_0().func3_interface_or_timer());
    p.timer32_1.ch1_cntr().modify(|_, w| w
        .mode().pwm()
        .en().set_bit()
    );
    p.timer32_1.top().modify(|_, w| unsafe { w.tim_top().bits(8000) });
    p.timer32_1.enable().modify(|_, w| w.tim_clr().set_bit());
    p.timer32_1.enable().modify(|_, w| w.tim_en().enable());
}

fn uart_init(p: &Peripherals) {
    p.pm.clk_apb_p_set().modify({|_, w| w
        .uart_0().enable()
    });

    p.pad_config.pad0_cfg().modify(|_, w| w.port_0_6().func2_interface());
    p.pad_config.pad0_cfg().modify(|_, w| w.port_0_5().func2_interface());
    p.usart_0.divider().modify(|_, w| unsafe { w.brr().bits(0xd05) });
    p.usart_0.control1().modify(|_, w| w
        .te().enable()
        .re().enable()
        .ue().enable()
    );

    while p.usart_0.flags().read().teack().bit_is_clear() {};
}

fn uart_send_bytes(p: &Peripherals, bytes: &[u8]) {
    for &byte in bytes {
        p.usart_0.txdata().modify(|_, w| unsafe { w.tdr().bits(byte.into()) });
        while p.usart_0.flags().read().tc().bit_is_clear() {};
    }
    p.usart_0.txdata().modify(|_, w| unsafe { w.tdr().bits(b'\n'.into()) });
    while p.usart_0.flags().read().tc().bit_is_clear() {};
}

fn uart_recieve_bytes(p: &Peripherals, buf: &mut [u8]) -> usize {
    let mut i = 0;

    loop {
        while p.usart_0.flags().read().rxne().bit_is_clear() {}
        let byte = p.usart_0.rxdata().read().rdr().bits() as u8;

        if byte == b'\n' {
            break;
        }

        if i < buf.len() {
            buf[i] = byte;
            i += 1;
        }
    }

    i
}

fn bytes_to_number(arr: &[u8]) -> u32 {
    let mut result = 0;
    for &digit in arr {
        if digit < b'0' || digit > b'9' { 
            continue;
        }
        result = result * 10 + (digit - b'0') as u32;
    }
    result
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    timer_init(&p);
    uart_init(&p);

    let bytes = "start".as_bytes();
    uart_send_bytes(&p, bytes);

    let mut pulse_width = scale(0);
    p.timer32_1.ch1_ocr().modify(|_, w| unsafe { w.ocr().bits(pulse_width) }); 

    let mut buf = [0; 10];

    loop {
        let len = uart_recieve_bytes(&p, &mut buf); 
        uart_send_bytes(&p, &buf[..len]);
        let num = bytes_to_number(&buf[..len]);

        let delay = 10000;

        let new_pulse_width = scale(num);
        if new_pulse_width > pulse_width {
            while pulse_width < new_pulse_width {
                pulse_width += 10;
                p.timer32_1.ch1_ocr().modify(|_, w| unsafe { w.ocr().bits(pulse_width) }); 
                riscv::asm::delay(delay);
            }
            riscv::asm::delay(delay);
            pulse_width = new_pulse_width;
            p.timer32_1.ch1_ocr().modify(|_, w| unsafe { w.ocr().bits(pulse_width) }); 
        } else {
            while pulse_width > new_pulse_width {
                pulse_width -= 10;
                p.timer32_1.ch1_ocr().modify(|_, w| unsafe { w.ocr().bits(pulse_width) }); 
                riscv::asm::delay(delay);
            }
            riscv::asm::delay(delay);
            pulse_width = new_pulse_width;
            p.timer32_1.ch1_ocr().modify(|_, w| unsafe { w.ocr().bits(pulse_width) }); 
        }
    }
}


#[unsafe(export_name = "trap_handler")]
fn trap() {
    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

