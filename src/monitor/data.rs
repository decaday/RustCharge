use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PortData {
    Unuse,
    Input(u32),
    Output(u32),
}

pub struct Data {
    pub battery_percentage: u8,
    pub battery_voltage_mv: u32,
    pub powerbank_current_ma: PortData,
    pub light_current_ma: PortData,
    pub brightness_percentage: u8,
    // pub output1_voltage_mills_data: PortData,
    // pub output2_voltage_mills_data: PortData,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            battery_percentage: 0,
            battery_voltage_mv: 0,
            powerbank_current_ma: PortData::Unuse,
            light_current_ma: PortData::Unuse,
            brightness_percentage: 0,
            // output1_voltage_mills_data: PortData::Unuse,
            // output2_voltage_mills_data: PortData::Unuse,
        }
    }
}

impl Data {
    pub fn get_battery_percentage_string(&self) -> String {
        let battery_percentage = self.battery_percentage;
        format!("{battery_percentage}%")
    }

    pub fn get_brightness_percentage_string(&self) -> String {
        let brightness_percentage = self.brightness_percentage;
        format!("{brightness_percentage}%")
    }

    pub fn get_battery_voltage_string(&self) -> String {
        let battery_voltage = self.battery_voltage_mv;
        mills_to_string(battery_voltage, 'V')
    }

    pub fn get_powerbank_power_string(&self, input_prefix: Option<&str>, output_prefix: Option<&str>) -> Option<String> {
        self.powerbank_current_ma.get_power_string(self.battery_voltage_mv, input_prefix, output_prefix)
    }

    pub fn get_powerbank_current_string(&self, input_prefix: Option<&str>, output_prefix: Option<&str>) -> Option<String> {
        self.powerbank_current_ma.get_string(input_prefix, output_prefix)
    }

    pub fn get_light_power_string(&self, input_prefix: Option<&str>, output_prefix: Option<&str>) -> Option<String> {
        self.light_current_ma.get_power_string(self.battery_voltage_mv, input_prefix, output_prefix)
    }

    pub fn get_light_current_string(&self, input_prefix: Option<&str>, output_prefix: Option<&str>) -> Option<String> {
        self.light_current_ma.get_string(input_prefix, output_prefix)
    }

    pub fn get_icons_list(&self) -> [screen::Icon; 3] {
        // Initialize with empty icons
        let mut icons = [screen::Icon::Unuse; 3]; 

    
        if let PortData::Input(_) = self.powerbank_current_ma {
            icons[0] = screen::Icon::Charging;
        }
    
        if let PortData::Output(current) = self.powerbank_current_ma {
            if self.battery_voltage_mv * current / 100 < 5000 {
                icons[1] = screen::Icon::Thunder;
            } else {
                icons[2] = screen::Icon::DoubleThunder;
            }
        }
    
        icons
    }
}

impl PortData {
    pub fn get_string(&self, input_prefix: Option<&str>, output_prefix: Option<&str>) -> Option<String> {
        match *self {
            PortData::Input(data) => {
                let string = mills_to_string(data, 'V');
                if let Some(prefix) = input_prefix {
                    Some(format!("{prefix}{string}"))
                }
                else {
                    Some(string) 
                }
            },
            PortData::Output(data) => {
                let string = mills_to_string(data, 'V');
                if let Some(prefix) = output_prefix {
                    Some(format!("{prefix}{string}"))
                }
                else {
                    Some(string) 
                }
            },
            PortData::Unuse => None,
        }
    }

    pub(crate) fn get_power_string(&self, mill_voltage_or_current: u32, input_prefix: Option<&str>, output_prefix: Option<&str>) -> Option<String> {        
        match *self {
            PortData::Input(data) => {
                let string = mills_to_power_string(data, mill_voltage_or_current);

                if let Some(prefix) = input_prefix {
                    Some(format!("{prefix}{string}"))
                }
                else {
                    Some(string) 
                }
            },
            PortData::Output(data) => {
                let string = mills_to_power_string(data, mill_voltage_or_current);
                if let Some(prefix) = output_prefix {
                    Some(format!("{prefix}{string}"))
                }
                else {
                    Some(string) 
                }
            },
            PortData::Unuse => None,
        }
    }

    pub fn new(data: i32, critical_value: u32) -> Self {
        if data < critical_value as i32 {
            PortData::Input(data as u32)
        } data > critical_value as i32 {
            PortData::Output(data as u32)
        }
        else {
            PortData::Unuse
        }
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

fn mills_to_power_string(mill_voltage: u32, mill_current: u32) -> String {
    let mill_power = mill_current * mill_voltage / 1000;
    mills_to_string(mill_power, 'W')
}
