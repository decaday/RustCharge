pub mod screen;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PortState {
    Unuse,
    Input,
    Output,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PortData {
    pub state: PortState,
    pub data: u32,
}

pub struct Data {
    pub battery_percentage: u8,
    pub battery_voltage_mills_data: u32,
    pub powerbank_current_ma: PortData,
    pub led_current_ma: PortData,
    pub output1_voltage_mills_data: PortData,
    pub output2_voltage_mills_data: PortData,
}

impl Data {
    pub fn get_battery_percentage_string(&self) -> String {
        let battery_percentage = self.battery_percentage;
        format!("{battery_percentage}%")
    }

    pub fn get_battery_voltage_string(&self) -> String {
        let battery_voltage = self.battery_voltage_mills_data;
        mills_to_string(battery_voltage, 'V')
    }
}

fn mills_to_string(mills_data: u32, unit: char) -> String {
    let data = mills_data / 1000;
    let mills_data_remainder = mills_data % 1000;
    
    let result = if mills_data_remainder == 0 {
        format!("{data}.00{unit}")
    } else {
        let mills_data_hundredths = mills_data_remainder / 100;
        let mills_data_remainder = (mills_data_remainder - mills_data_hundredths * 100) / 10;
    
        format!("{data}.{mills_data_hundredths}{mills_data_remainder}{unit}")
    };
    
    result
}
