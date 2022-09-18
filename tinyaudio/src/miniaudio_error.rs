use miniaudio_sys::*;
use std::error::Error;
use std::fmt::Display;

#[repr(C)]
#[derive(Debug)]
pub enum MiniaudioError {
    Error = MA_ERROR as _,
    InvalidArgs = MA_INVALID_ARGS as _,
    InvalidOperation = MA_INVALID_OPERATION as _,
    OutOfMemory = MA_OUT_OF_MEMORY as _,
    OutOfRange = MA_OUT_OF_RANGE as _,
    AccessDenied = MA_ACCESS_DENIED as _,
    DoesNotExist = MA_DOES_NOT_EXIST as _,
    AlreadyExists = MA_ALREADY_EXISTS as _,
    TooManyOpenFiles = MA_TOO_MANY_OPEN_FILES as _,
    InvalidFile = MA_INVALID_FILE as _,
    TooBig = MA_TOO_BIG as _,
    PathTooLong = MA_PATH_TOO_LONG as _,
    NameTooLong = MA_NAME_TOO_LONG as _,
    NotDirectory = MA_NOT_DIRECTORY as _,
    IsDirectory = MA_IS_DIRECTORY as _,
    DirectoryNotEmpty = MA_DIRECTORY_NOT_EMPTY as _,
    AtEnd = MA_AT_END as _,
    NoSpace = MA_NO_SPACE as _,
    Busy = MA_BUSY as _,
    IoError = MA_IO_ERROR as _,
    Interrupt = MA_INTERRUPT as _,
    Unavailable = MA_UNAVAILABLE as _,
    AlreadyInUse = MA_ALREADY_IN_USE as _,
    BadAddress = MA_BAD_ADDRESS as _,
    BadSeek = MA_BAD_SEEK as _,
    BadPipe = MA_BAD_PIPE as _,
    Deadlock = MA_DEADLOCK as _,
    TooManyLinks = MA_TOO_MANY_LINKS as _,
    NotImplemented = MA_NOT_IMPLEMENTED as _,
    NoMessage = MA_NO_MESSAGE as _,
    BadMessage = MA_BAD_MESSAGE as _,
    NoDataAvailable = MA_NO_DATA_AVAILABLE as _,
    InvalidData = MA_INVALID_DATA as _,
    Timeout = MA_TIMEOUT as _,
    NoNetwork = MA_NO_NETWORK as _,
    NotUnique = MA_NOT_UNIQUE as _,
    NotSocket = MA_NOT_SOCKET as _,
    NoAddress = MA_NO_ADDRESS as _,
    BadProtocol = MA_BAD_PROTOCOL as _,
    ProtocolUnavailable = MA_PROTOCOL_UNAVAILABLE as _,
    ProtocolNotSupported = MA_PROTOCOL_NOT_SUPPORTED as _,
    ProtocolFamilyNotSupported = MA_PROTOCOL_FAMILY_NOT_SUPPORTED as _,
    AddressFamilyNotSupported = MA_ADDRESS_FAMILY_NOT_SUPPORTED as _,
    SocketNotSupported = MA_SOCKET_NOT_SUPPORTED as _,
    ConnectionReset = MA_CONNECTION_RESET as _,
    AlreadyConnected = MA_ALREADY_CONNECTED as _,
    NotConnected = MA_NOT_CONNECTED as _,
    ConnectionRefused = MA_CONNECTION_REFUSED as _,
    NoHost = MA_NO_HOST as _,
    InProgress = MA_IN_PROGRESS as _,
    Cancelled = MA_CANCELLED as _,
    MemoryAlreadyMapped = MA_MEMORY_ALREADY_MAPPED as _,
    FormatNotSupported = MA_FORMAT_NOT_SUPPORTED as _,
    DeviceTypeNotSupported = MA_DEVICE_TYPE_NOT_SUPPORTED as _,
    ShareModeNotSupported = MA_SHARE_MODE_NOT_SUPPORTED as _,
    NoBackend = MA_NO_BACKEND as _,
    NoDevice = MA_NO_DEVICE as _,
    ApiNotFound = MA_API_NOT_FOUND as _,
    InvalidDeviceConfig = MA_INVALID_DEVICE_CONFIG as _,
    Loop = MA_LOOP as _,
    DeviceNotInitialized = MA_DEVICE_NOT_INITIALIZED as _,
    DeviceAlreadyInitialized = MA_DEVICE_ALREADY_INITIALIZED as _,
    DeviceNotStarted = MA_DEVICE_NOT_STARTED as _,
    DeviceNotStopped = MA_DEVICE_NOT_STOPPED as _,
    FailedToInitBackend = MA_FAILED_TO_INIT_BACKEND as _,
    FailedToOpenBackendDevice = MA_FAILED_TO_OPEN_BACKEND_DEVICE as _,
    FailedToStartBackendDevice = MA_FAILED_TO_START_BACKEND_DEVICE as _,
    FailedToStopBackendDevice = MA_FAILED_TO_STOP_BACKEND_DEVICE as _,
}

impl Display for MiniaudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "miniaudio error: {self:#?}")
    }
}

impl Error for MiniaudioError {}

#[macro_export]
macro_rules! ma_result {
    ($Result:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            match $Result {
                miniaudio_sys::MA_SUCCESS => Ok(()),
                err => Err(std::mem::transmute::<
                    miniaudio_sys::ma_result,
                    crate::miniaudio_error::MiniaudioError,
                >(err)),
            }
        }
    }};
}
