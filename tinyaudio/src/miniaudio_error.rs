use miniaudio_sys::*;
use std::error::Error;
use std::fmt::Display;

#[repr(i32)]
#[derive(Debug)]
pub enum MiniaudioError {
    Error = MA_ERROR,
    InvalidArgs = MA_INVALID_ARGS,
    InvalidOperation = MA_INVALID_OPERATION,
    OutOfMemory = MA_OUT_OF_MEMORY,
    OutOfRange = MA_OUT_OF_RANGE,
    AccessDenied = MA_ACCESS_DENIED,
    DoesNotExist = MA_DOES_NOT_EXIST,
    AlreadyExists = MA_ALREADY_EXISTS,
    TooManyOpenFiles = MA_TOO_MANY_OPEN_FILES,
    InvalidFile = MA_INVALID_FILE,
    TooBig = MA_TOO_BIG,
    PathTooLong = MA_PATH_TOO_LONG,
    NameTooLong = MA_NAME_TOO_LONG,
    NotDirectory = MA_NOT_DIRECTORY,
    IsDirectory = MA_IS_DIRECTORY,
    DirectoryNotEmpty = MA_DIRECTORY_NOT_EMPTY,
    AtEnd = MA_AT_END,
    NoSpace = MA_NO_SPACE,
    Busy = MA_BUSY,
    IoError = MA_IO_ERROR,
    Interrupt = MA_INTERRUPT,
    Unavailable = MA_UNAVAILABLE,
    AlreadyInUse = MA_ALREADY_IN_USE,
    BadAddress = MA_BAD_ADDRESS,
    BadSeek = MA_BAD_SEEK,
    BadPipe = MA_BAD_PIPE,
    Deadlock = MA_DEADLOCK,
    TooManyLinks = MA_TOO_MANY_LINKS,
    NotImplemented = MA_NOT_IMPLEMENTED,
    NoMessage = MA_NO_MESSAGE,
    BadMessage = MA_BAD_MESSAGE,
    NoDataAvailable = MA_NO_DATA_AVAILABLE,
    InvalidData = MA_INVALID_DATA,
    Timeout = MA_TIMEOUT,
    NoNetwork = MA_NO_NETWORK,
    NotUnique = MA_NOT_UNIQUE,
    NotSocket = MA_NOT_SOCKET,
    NoAddress = MA_NO_ADDRESS,
    BadProtocol = MA_BAD_PROTOCOL,
    ProtocolUnavailable = MA_PROTOCOL_UNAVAILABLE,
    ProtocolNotSupported = MA_PROTOCOL_NOT_SUPPORTED,
    ProtocolFamilyNotSupported = MA_PROTOCOL_FAMILY_NOT_SUPPORTED,
    AddressFamilyNotSupported = MA_ADDRESS_FAMILY_NOT_SUPPORTED,
    SocketNotSupported = MA_SOCKET_NOT_SUPPORTED,
    ConnectionReset = MA_CONNECTION_RESET,
    AlreadyConnected = MA_ALREADY_CONNECTED,
    NotConnected = MA_NOT_CONNECTED,
    ConnectionRefused = MA_CONNECTION_REFUSED,
    NoHost = MA_NO_HOST,
    InProgress = MA_IN_PROGRESS,
    Cancelled = MA_CANCELLED,
    MemoryAlreadyMapped = MA_MEMORY_ALREADY_MAPPED,
    FormatNotSupported = MA_FORMAT_NOT_SUPPORTED,
    DeviceTypeNotSupported = MA_DEVICE_TYPE_NOT_SUPPORTED,
    ShareModeNotSupported = MA_SHARE_MODE_NOT_SUPPORTED,
    NoBackend = MA_NO_BACKEND,
    NoDevice = MA_NO_DEVICE,
    ApiNotFound = MA_API_NOT_FOUND,
    InvalidDeviceConfig = MA_INVALID_DEVICE_CONFIG,
    Loop = MA_LOOP,
    DeviceNotInitialized = MA_DEVICE_NOT_INITIALIZED,
    DeviceAlreadyInitialized = MA_DEVICE_ALREADY_INITIALIZED,
    DeviceNotStarted = MA_DEVICE_NOT_STARTED,
    DeviceNotStopped = MA_DEVICE_NOT_STOPPED,
    FailedToInitBackend = MA_FAILED_TO_INIT_BACKEND,
    FailedToOpenBackendDevice = MA_FAILED_TO_OPEN_BACKEND_DEVICE,
    FailedToStartBackendDevice = MA_FAILED_TO_START_BACKEND_DEVICE,
    FailedToStopBackendDevice = MA_FAILED_TO_STOP_BACKEND_DEVICE,
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
