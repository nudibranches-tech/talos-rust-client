// Generated proto modules

// Google proto types need to be available for the generated code
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
    pub mod rpc {
        include!("google.rpc.rs");
    }
}

// Include all generated modules here so they can reference super::google
pub mod common {
    include!("common.rs");
}

pub mod machine {
    include!("machine.rs");
}

pub mod cluster {
    include!("cluster.rs");
}

pub mod inspect {
    include!("inspect.rs");
}

pub mod securityapi {
    include!("securityapi.rs");
}

pub mod storage {
    include!("storage.rs");
}

pub mod time {
    include!("time.rs");
}