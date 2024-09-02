pub mod screen;



pub struct PortData {
    pub state: bool,
    pub data: i32,
}

pub struct Data {
    pub battery_percentage: u8,
    pub battery_voltage_mv: u32,
    pub powerbank_current_ma: PortData,
    pub led_current_ma: PortData,
    pub output1_voltage_mv: PortData,
    pub output2_voltage_mv: PortData,
}
