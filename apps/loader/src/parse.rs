
use std::vec::Vec;

pub struct Header<const APP_NUM: usize, const IMAGE_START: usize> {
    pub apps_start: Vec<usize>,
}

impl<const APP_NUM: usize, const IMAGE_START: usize> Header<APP_NUM, IMAGE_START> {
    fn new() -> Self {
        Header {
            apps_start: vec![],
        }
    }

    pub fn get_app_lens() -> Self {
        const USIZE_LEN: usize = core::mem::size_of::<usize>();
        let app0_start =  IMAGE_START + USIZE_LEN * APP_NUM;
        let mut apps_start = vec![app0_start];

        let mut base = 0;
        
        for i in 0..APP_NUM {
            let bytes = unsafe { 
                core::slice::from_raw_parts((IMAGE_START + base) as *const u8 , USIZE_LEN) 
            };

            let pre_app_start = apps_start.last().unwrap();
            let app_len = usize::from_be_bytes(bytes.try_into().unwrap());
            apps_start.push(pre_app_start + app_len);
        }


        Self {
            apps_start
        }
    }

    pub fn app_len(&self, app_num: usize) -> usize {
        self.apps_start[app_num + 1] - self.apps_start[app_num]
    }

    pub fn app_start(&self, app_num: usize) -> usize {
        self.apps_start[app_num]
    }
}
