# Real-Time-Control-System

## Overview

This project implements a real-time control system for a motor in pure Rust.


## Objective

Determine how fast the moter can go and measure the latency and jitter of the control system.  


## Hardware

MCP: [Rasperry Pi Pico](https://www.amazon.com/dp/B09KVB8LVR?ref=ppx_yo2ov_dt_b_fed_asin_title)  
Motor: [12V DC Gear Motor with Encoder](https://www.amazon.com/dp/B08JLDHPMJ?ref=ppx_yo2ov_dt_b_fed_asin_title)  
Motor Driver: [TB6612FNG](https://www.amazon.com/dp/B09MN94LVF?ref=ppx_yo2ov_dt_b_fed_asin_title)  
Power Supply: [7.4V Li-ion Battery Pack](https://www.amazon.com/dp/B09V7HQYSZ?ref=ppx_yo2ov_dt_b_fed_asin_title)  
Logic Analyzer: [HiLetgo USB Logic Analyzer](https://www.amazon.com/dp/B077LSG5P2?ref=ppx_yo2ov_dt_b_fed_asin_title)  
Breadboard: [BOJACK Solderless Breadboard Kit](https://www.amazon.com/dp/B08Y59P6D1?ref=ppx_yo2ov_dt_b_fed_asin_title)  

## Core Rust Crates Used

- Rust
- Embedded Rust
- no_std

## Features Implemented

- 


## Milestones

1. Get microcontroller board alive, toolchain set up, and basic peripherals working.
> - Choose Hardware
> - Blink LED on timer interrupt
> - UART/USB logging
> - Confirm Motor Driver Circuit Works
2. Control motor speed with open-loop PWM and measure feedback.
> - Configure PWM output to drive motor driver at varying speeds.
> - Set up encoder input.
> - Read motor current via ADC.
> - Log data over UART for debugging.
3. Establish deterministic timing (real-time backbone).
> - Configure hardware timer -> 1kHz interrupt.
> - Is ISR: read encoder, update PID, adjust PWM.
> - Measure execution time.
> - Output: Verify ISR latency and jitter (e.g. +- 5-10us).
4. Close the loop with control math.
> - Implement PID (Proportional-Integral-Derivative) control algorithm.
> - Tune 
5. Make system robust like production firmware.
6. Package firmware and demo into a clear, polished project.

## Current Project Status (Last Updated: 09/10/2025)

- Milestone 1: Hardware parts have been ordered and are being shipped.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
