#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const P4LIBRARIES_INIT_P4: i32 = 0x01;
pub const P4LIBRARIES_INIT_SQLITE: i32 = 0x02;
pub const P4LIBRARIES_INIT_CURL: i32 = 0x04;
pub const P4LIBRARIES_INIT_OPENSSL: i32 = 0x08;
pub const P4LIBRARIES_INIT_ALL: i32 = 0x0F;

pub mod easy {

    pub fn connect() {

    }

    pub  fn run() {

    }
}

#[cfg(test)]

mod tests {

    use std::{fmt::Debug, string};

    use super::*;

    fn to_i8_ptr(input: &str) -> &[i8] {
        
        unsafe {
            &*(input.as_bytes() as *const _ as *const [i8])
        }
    }

    fn strbuf_to_string(input: &StrBuf) -> String {

         input._base.buffer;
        "test".to_string()
    }

    #[test]
    fn test_01() {
        unsafe {

            let mut e = P4Error::new()._base;
            let mut msg = P4StrBuf::new()._base;

            P4Libraries::Initialize(P4LIBRARIES_INIT_ALL, &mut e);
            {
                e.Fmt( &mut msg, 0);
                println!("Error: {}", "error");
            }

            let mut ui = ClientUser::new(-1, 0);
            let mut client = ClientApi::new();

            let port = to_i8_ptr("127.0.0.1:1666\0").as_ptr();
            let user = "p4admin\0";
            let passwd = "pass12349ers\0";

            client.SetPort(port);
            client.SetUser(to_i8_ptr(user).as_ptr());
            client.SetPassword(to_i8_ptr(passwd).as_ptr());

            client.Init(&mut e);

            if e.severity > ErrorSeverity_E_INFO {
                
                e.Fmt( &mut msg, 0);
                println!("Error: {:?}", strbuf_to_string(msg));
            }

            //client.SetArgv( argc - 2, argv + 2 );
            let args = to_i8_ptr("info\0").as_ptr();
            client.Run( args, &mut ui);
            
            
            client.Final(&mut e);

            P4Libraries::Shutdown(P4LIBRARIES_INIT_ALL, &mut e);
        }
    }
}
