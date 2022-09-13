/// Macro to conditionally call [tracing::trace] if tracing is enabled
#[macro_export]
macro_rules! trace_if {
    ($($tts:tt)*) => {
        {
            #[cfg(feature = "tracing")]
            {
                ::tracing::trace!($($tts)*);
            }
        }
    }
}

/// Macro to conditionally call [tracing::debug] if tracing is enabled
#[macro_export]
macro_rules! debug_if {
    ($($tts:tt)*) => {
        {
            #[cfg(feature = "tracing")]
            {
                ::tracing::debug!($($tts)*);
            }
        }
    }
}
