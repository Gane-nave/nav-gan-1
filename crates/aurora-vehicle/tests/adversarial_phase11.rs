//! Adversarial integration tests for Phase 11 bug fixes and vehicle features.

use aurora_vehicle::adapter::Calibration;
use aurora_vehicle::protocol::{ObdPid, ProtocolHandler};

// ============================================================
// BUG FIX 1: map_axes axis=0 → 0.0 (not input[0])
// BROKEN code: saturating_sub(1) on axis=0 gives idx=0, same as axis=1
// FIXED code: axis=0 means "no mapping", output stays 0.0
// ============================================================

#[test]
fn adversarial_map_axes_all_zero() {
    let cal = Calibration {
        axis_map: [0, 0, 0],
        ..Default::default()
    };
    let output = cal.map_axes([10.0, 20.0, 30.0]);
    // BROKEN: [10.0, 10.0, 10.0]  FIXED: [0.0, 0.0, 0.0]
    assert_eq!(
        output,
        [0.0, 0.0, 0.0],
        "axis_map=[0,0,0] must output [0,0,0], not [10,10,10]"
    );
}

#[test]
fn adversarial_map_axes_partial_zero() {
    let cal = Calibration {
        axis_map: [0, 2, -3],
        ..Default::default()
    };
    let output = cal.map_axes([10.0, 20.0, 30.0]);
    assert!((output[0]).abs() < 0.001, "axis=0 → 0.0, got {}", output[0]);
    assert!(
        (output[1] - 20.0).abs() < 0.001,
        "axis=2 → input[1]=20.0, got {}",
        output[1]
    );
    assert!(
        (output[2] + 30.0).abs() < 0.001,
        "axis=-3 → -input[2]=-30.0, got {}",
        output[2]
    );
}

// ============================================================
// OBD-II decoding formulas verification
// ============================================================

#[test]
fn adversarial_obd_rpm_decode() {
    // RPM = (A*256 + B) / 4 = (26*256 + 248)/4 = 6904/4 = 1726.0
    let rpm = ProtocolHandler::decode_obd(ObdPid::EngineRpm, &[0x1A, 0xF8]).unwrap();
    assert!(
        (rpm - 1726.0).abs() < 0.01,
        "RPM should be 1726.0, got {}",
        rpm
    );
}

#[test]
fn adversarial_obd_coolant_temp() {
    // Temp = A - 40 = 160 - 40 = 120°C
    let temp = ProtocolHandler::decode_obd(ObdPid::CoolantTemp, &[0xA0]).unwrap();
    assert!(
        (temp - 120.0).abs() < 0.01,
        "Temp should be 120.0°C, got {}",
        temp
    );
}

#[test]
fn adversarial_obd_speed() {
    // Speed = A = 100 km/h
    let speed = ProtocolHandler::decode_obd(ObdPid::VehicleSpeed, &[0x64]).unwrap();
    assert!(
        (speed - 100.0).abs() < 0.01,
        "Speed should be 100.0 km/h, got {}",
        speed
    );
}

#[test]
fn adversarial_obd_fuel_full() {
    // Fuel = A * 100 / 255 = 255*100/255 = 100.0%
    let fuel = ProtocolHandler::decode_obd(ObdPid::FuelLevel, &[0xFF]).unwrap();
    assert!(
        (fuel - 100.0).abs() < 0.01,
        "Fuel should be 100.0%, got {}",
        fuel
    );
}
