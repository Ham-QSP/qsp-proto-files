pub mod qsp {
    pub mod example {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/qsp.trxcontrol.v1.rs"));
        }
    }
}
