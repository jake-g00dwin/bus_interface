// The data that gets returned from the command requests.
#![allow(dead_code)]


//considering the use of this C style union.
//it requires use of 'unsafe' and may not be the best choice.
#[repr(C)]
union UData {
    du8: u8,
    du16: u16,
    du32: u32,
    di8: i8,
    di16: i16,
    di32: i32,
    df32: f32,
}

#[derive(Debug)]
pub struct CmdReturn {
    name: String,
    format: Vec<String>,
    data_names: Vec<String>,
    raw_bytes: Vec<u8>,
}

impl CmdReturn {
    pub fn new() -> CmdReturn {
        let ret = CmdReturn{
            name: String::new(),
            format: vec![],
            data_names: vec![], 
            raw_bytes: vec![],
        };
        ret
    }  

    pub fn parse_to_json(&self) -> String {
        let mut json = String::new();
        //1. add the name json.
        json.push_str("{\"name\":\"");
        json.push_str(&self.name);
        json.push_str("\"");

        let data_strings = self.bytes_to_strings();

        //2. Add the data.
        for i in 0..self.data_names.len() {
            json.push_str(", \"");
            json.push_str(&self.data_names[i]);
            json.push_str("\":\"");
            json.push_str(&data_strings[i]);
            json.push_str("\"");
        }
        
        json.push_str("}");

        json
    }

    fn bytes_to_strings(&self) -> Vec<String> {
        let mut data: Vec<String> = vec![]; 
        let mut byte_index: usize = 0;
        
        for fmt in &self.format {
            if fmt.contains("u8") {
                data.push(format!("{}",self.raw_bytes[byte_index]));    
                byte_index += 1;
            }
            else if fmt.contains("u16") {
                let tmp: u16 = bytes_to_u16(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp));    
                byte_index += 2;
            }

            else if fmt.contains("i16") {
                let tmp: i16 = bytes_to_i16(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp));    
                byte_index += 2;
            }

            else if fmt.contains("u32") {
                let tmp: u32 = bytes_to_u32(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp)); 
                byte_index += 4;
            }

            else if fmt.contains("i32") {
                let tmp: i32 = bytes_to_i32(&self.raw_bytes, byte_index);
                data.push(format!("{}",tmp)); 
                byte_index += 4;
            }
        }

        data
    }

}

fn bytes_to_u16(b: &Vec<u8>, start: usize) -> u16 {
        let tmp: u16 =
            ((b[start] as u16 )<< 8) | 
            (b[start + 1] as u16);
        tmp
}


fn bytes_to_i16(b: &Vec<u8>, start: usize) -> i16 {
        let tmp: i16 =
            ((b[start] as i16 )<< 8) | 
            (b[start + 1] as i16);
        tmp
}


fn bytes_to_u32(b: &Vec<u8>, start: usize) -> u32 {
    let tmp: u32 = 
        ((b[start] as u32 )<< 24) | 
        ((b[start + 1] as u32 )<< 16) | 
        ((b[start + 2] as u32 )<< 8) | 
        (b[start + 3] as u32);
        tmp
}


fn bytes_to_i32(b: &Vec<u8>, start: usize) -> i32 {
    let tmp: i32 = 
        ((b[start] as i32 )<< 24) | 
        ((b[start + 1] as i32 )<< 16) | 
        ((b[start + 2] as i32 )<< 8) | 
        (b[start + 3] as i32);
        tmp
}

//Tests for the structure
mod test_cmdreturn {
    #![allow(unused_imports)]
    use super::*;
    
    fn setup() -> CmdReturn {
        let mut new_response = CmdReturn::new();
        new_response.name = String::from("aht20");
        new_response.format.push(String::from("u16"));
        new_response.format.push(String::from("u16"));
        new_response.data_names.push(String::from("Temp"));
        new_response.data_names.push(String::from("Humid"));
        new_response.raw_bytes = vec!(0, 255, 0, 255);

        new_response
    }

    #[test]
    fn self_test() {
        assert!(true);
    }


    #[test]
    fn test_name() {
        let mut new_response = CmdReturn::new();
        new_response.name = String::from("fake_sensor");
        assert_eq!(new_response.name, String::from("fake_sensor"));
    }


    #[test]
    fn test_parse_to_json() {
        let ret = setup();
        let correct_response = String::from("{\"name\":\"aht20\", \"Temp\":\"255\", \"Humid\":\"255\"}");
        
        //test list
        //1. has parse function.
        let json_str = ret.parse_to_json();

        //2. outputs jason correctly.
        assert_eq!(json_str, correct_response);
    }
}
