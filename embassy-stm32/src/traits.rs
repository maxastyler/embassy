#![macro_use]

macro_rules! pin_trait {
    ($signal:ident, $instance:path) => {
        pub trait $signal<T: $instance>: crate::gpio::Pin {
            fn af_num(&self) -> u8;
        }
    };
}

macro_rules! pin_trait_impl {
    ($signal:ident, $instance:ident, $pin:ident, $af:expr) => {
        impl $signal<crate::peripherals::$instance> for crate::peripherals::$pin {
            fn af_num(&self) -> u8 {
                $af
            }
        }
    };
}

// ====================

macro_rules! dma_trait {
    ($signal:ident, $instance:path) => {
        pub trait $signal<T: $instance>: crate::dma::Channel {
            fn request(&self) -> crate::dma::Request;
        }
    };
}

#[allow(unused)]
macro_rules! dma_trait_impl {
    // DMAMUX
    ($signal:ident, $instance:ident, {dmamux: $dmamux:ident}, $request:expr) => {
        impl<T> $signal<crate::peripherals::$instance> for T
        where
            T: crate::dma::MuxChannel<Mux = crate::dma::$dmamux>,
        {
            fn request(&self) -> crate::dma::Request {
                $request
            }
        }
    };

    // No DMAMUX
    ($signal:ident, $instance:ident, {channel: $channel:ident}, $request:expr) => {
        impl $signal<crate::peripherals::$instance> for crate::peripherals::$channel {
            fn request(&self) -> crate::dma::Request {
                $request
            }
        }
    };
}