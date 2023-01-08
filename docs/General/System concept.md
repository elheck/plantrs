[[_Dashboard]]

### General
- The basic idea of the system is to create a fully automatic hydroponics grow system, which will live in an enclosed environment so that every factor can be controlled.
- The system is implemented in Rust using a [[Raspberry Pi Pico]] micro controller.
- A [[Recipe]] has to be created which will configure the hardware and runtime of the system
- The runtime ([[Runtime Moc]]) has to compensate some user errors and some adjustments to better suite plant [[Growing Phase]]s

### Calibration
- Because many sensors need real calibration every grow period, a [[Calibration Hardware]] and [[Calibration Software]] is used to accomplish this.