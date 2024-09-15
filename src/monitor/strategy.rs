use super::*;


pub fn get_battery_percentage(data: &Data) -> u8 {
    let battery_energy_mwh = BATTERY_ENERGY_MWH.load(Ordering::Release);
    let battery_energy_max_mwh = BATTERY_MAX_ENERGY_MWH.load(Ordering::Relaxed);

    let battery_percentage = ((battery_energy_mwh * 100) / battery_energy_max_mwh) as u8;
    let battery_voltage_mv = data.battery_voltage_mv;
    let powerbank_current_ma = data.powerbank_current_ma;

    if data.input_level {
        if battery_voltage_mv > 4100 {
            if PortData::Unuse == powerbank_current_ma {
                BATTERY_MAX_ENERGY_MWH.store(battery_energy_mwh, Ordering::Release);
                return 100
            }
        }
    }

    if battery_voltage_mv < 3300 {
        BATTERY_ENERGY_MWH.store(battery_energy_max_mwh / 100, Ordering::Release);
        return 1
    }

    battery_percentage
}