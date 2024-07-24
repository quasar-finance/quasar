pub mod osmosis {
    pub mod epochs {
        pub mod v1beta1 {
            include!("osmosis.epochs.v1beta1.rs");
        }
    }
    pub mod gamm {
        pub mod poolmodels {
            pub mod balancer {
                pub mod v1beta1 {
                    include!("osmosis.gamm.poolmodels.balancer.v1beta1.rs");
                }
            }
        }
        pub mod v1beta1 {
            include!("osmosis.gamm.v1beta1.rs");
        }
    }
    pub mod incentives {
        include!("osmosis.incentives.rs");
    }
    pub mod lockup {
        include!("osmosis.lockup.rs");
    }
    pub mod mint {
        pub mod v1beta1 {
            include!("osmosis.mint.v1beta1.rs");
        }
    }
    pub mod poolincentives {
        pub mod v1beta1 {
            include!("osmosis.poolincentives.v1beta1.rs");
        }
    }
}
pub mod quasarlabs {
    pub mod quasarnode {
        pub mod epochs {
            include!("quasarlabs.quasarnode.epochs.rs");
        }
        pub mod qoracle {
            pub mod osmosis {
                include!("quasarlabs.quasarnode.qoracle.osmosis.rs");
            }
            include!("quasarlabs.quasarnode.qoracle.rs");
        }
        pub mod qtransfer {
            include!("quasarlabs.quasarnode.qtransfer.rs");
        }
        pub mod qvesting {
            include!("quasarlabs.quasarnode.qvesting.rs");
        }
        pub mod tokenfactory {
            pub mod v1beta1 {
                include!("quasarlabs.quasarnode.tokenfactory.v1beta1.rs");
            }
        }
    }
}
