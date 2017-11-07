extern crate libloading;
//extern crate libc;

use std::ptr;
//use libc::c_uchar;

#[allow(non_camel_case_types)]
pub type CK_BYTE = u8;
#[allow(non_camel_case_types)]
pub type CK_CHAR = CK_BYTE;
#[allow(non_camel_case_types)]
pub type CK_UTF8CHAR = CK_BYTE;
#[allow(non_camel_case_types)]
pub type CK_BBOOL = CK_BYTE;
#[allow(non_camel_case_types)]
pub type CK_ULONG = usize;
#[allow(non_camel_case_types)]
pub type CK_ULONG_PTR = *const CK_ULONG;
#[allow(non_camel_case_types)]
pub type CK_LONG = isize;
#[allow(non_camel_case_types)]
pub type CK_FLAGS = CK_ULONG;
#[allow(non_camel_case_types)]
pub type CK_RV = CK_ULONG;
#[allow(non_camel_case_types)]
pub type CK_SLOT_ID = CK_ULONG;
#[allow(non_camel_case_types)]
pub type CK_SLOT_ID_PTR = *const CK_SLOT_ID;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
#[repr(u8)]
pub enum CK_VOID {
    #[doc(hidden)]
    __Variant1,
    #[doc(hidden)]
    __Variant2,
}

// TODO: in rust we could protect more with *const in a lot of cases
#[allow(non_camel_case_types, non_snake_case)]
pub type CK_VOID_PTR = *const CK_VOID;

#[allow(non_camel_case_types, non_snake_case)]
pub type CK_VOID_PTR_PTR = *const CK_VOID_PTR;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug,Clone)]
#[repr(C)]
pub struct CK_VERSION {
  pub major: CK_BYTE,  /* integer portion of version number */
  pub minor: CK_BYTE,   /* 1/100ths portion of version number */
}

#[allow(non_camel_case_types)]
pub type CK_CREATEMUTEX = Option<extern "C" fn(CK_VOID_PTR_PTR) -> CK_RV>;
#[allow(non_camel_case_types)]
pub type CK_DESTROYMUTEX = Option<extern "C" fn(CK_VOID_PTR) -> CK_RV>;
#[allow(non_camel_case_types)]
pub type CK_LOCKMUTEX = Option<extern "C" fn(CK_VOID_PTR) -> CK_RV>;
#[allow(non_camel_case_types)]
pub type CK_UNLOCKMUTEX = Option<extern "C" fn(CK_VOID_PTR) -> CK_RV>;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct CK_C_INITIALIZE_ARGS {
  pub CreateMutex: CK_CREATEMUTEX,
  pub DestroyMutex: CK_DESTROYMUTEX,
  pub LockMutex: CK_LOCKMUTEX,
  pub UnlockMutex: CK_UNLOCKMUTEX,
  pub flags: CK_FLAGS,
  pub pReserved: CK_VOID_PTR,
}

impl CK_C_INITIALIZE_ARGS {
    pub fn new() -> CK_C_INITIALIZE_ARGS {
        CK_C_INITIALIZE_ARGS {
            flags: CKF_OS_LOCKING_OK,
            CreateMutex: None,
            DestroyMutex: None,
            LockMutex: None,
            UnlockMutex: None,
            pReserved: ptr::null(),
        }
    }
}

pub const CKF_LIBRARY_CANT_CREATE_OS_THREADS: CK_FLAGS = 0x00000001;
pub const CKF_OS_LOCKING_OK: CK_FLAGS                  = 0x00000002;

#[allow(non_camel_case_types)]
pub type CK_C_INITIALIZE_ARGS_PTR = *const CK_C_INITIALIZE_ARGS;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct CK_INFO {
  /* manufacturerID and libraryDecription have been changed from
   * CK_CHAR to CK_UTF8CHAR for v2.10 */
  pub cryptokiVersion: CK_VERSION,              /* Cryptoki interface ver */
  pub manufacturerID: [CK_UTF8CHAR; 32],        /* blank padded */
  pub flags: CK_FLAGS,                          /* must be zero */

  pub libraryDescription: [CK_UTF8CHAR; 32],    /* blank padded */
  pub libraryVersion: CK_VERSION,               /* version of library */
}

impl CK_INFO {
    pub fn new() -> CK_INFO {
        CK_INFO {
            cryptokiVersion: CK_VERSION { major: 0, minor: 0 },
            manufacturerID: [0; 32],
            flags: 0,
            libraryDescription: [0; 32],
            libraryVersion: CK_VERSION { major: 0, minor: 0 },
        }
    }
}

#[allow(non_camel_case_types)]
pub type CK_INFO_PTR = *const CK_INFO;

#[allow(non_camel_case_types, non_snake_case)]
#[repr(C)]
pub struct CK_SLOT_INFO {
  /* slotDescription and manufacturerID have been changed from
   * CK_CHAR to CK_UTF8CHAR for v2.10 */
  pub slotDescription: [CK_UTF8CHAR; 64],    /* blank padded */
  pub manufacturerID: [CK_UTF8CHAR; 32],     /* blank padded */
  pub flags: CK_FLAGS,

  pub hardwareVersion: CK_VERSION,  /* version of hardware */
  pub firmwareVersion: CK_VERSION,  /* version of firmware */
}

impl std::fmt::Debug for CK_SLOT_INFO {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sd = {
            &self.slotDescription[..].fmt(fmt)
        };
        fmt.debug_struct("CK_SLOT_INFO")
            .field("slotDescription", &sd)
            .field("manufacturerID", &self.manufacturerID)
            .field("flags", &self.flags)
            .field("hardwareVersion", &self.hardwareVersion)
            .field("firmwareVersion", &self.firmwareVersion)
            .finish()
    }
}

#[allow(non_camel_case_types)]
pub type CK_SLOT_INFO_PTR = *const CK_INFO;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct CK_TOKEN_INFO {
  /* label, manufacturerID, and model have been changed from
   * CK_CHAR to CK_UTF8CHAR for v2.10 */
  pub label: [CK_UTF8CHAR; 32],           /* blank padded */
  pub manufacturerID: [CK_UTF8CHAR; 32],  /* blank padded */
  pub model: [CK_UTF8CHAR; 16],           /* blank padded */
  pub serialNumber: [CK_CHAR; 16],        /* blank padded */
  pub flags: CK_FLAGS,                    /* see below */

  pub ulMaxSessionCount: CK_ULONG,     /* max open sessions */
  pub ulSessionCount: CK_ULONG,        /* sess. now open */
  pub ulMaxRwSessionCount: CK_ULONG,   /* max R/W sessions */
  pub ulRwSessionCount: CK_ULONG,      /* R/W sess. now open */
  pub ulMaxPinLen: CK_ULONG,           /* in bytes */
  pub ulMinPinLen: CK_ULONG,           /* in bytes */
  pub ulTotalPublicMemory: CK_ULONG,   /* in bytes */
  pub ulFreePublicMemory: CK_ULONG,    /* in bytes */
  pub ulTotalPrivateMemory: CK_ULONG,  /* in bytes */
  pub ulFreePrivateMemory: CK_ULONG,   /* in bytes */
  pub hardwareVersion: CK_VERSION,     /* version of hardware */
  pub firmwareVersion: CK_VERSION,     /* version of firmware */
  pub utcTime: [CK_CHAR; 16],          /* time */
}

#[allow(non_camel_case_types)]
pub type CK_TOKEN_INFO_PTR = *const CK_TOKEN_INFO;

#[allow(non_camel_case_types)]
pub type CK_MECHANISM_TYPE = CK_ULONG;
#[allow(non_camel_case_types)]
pub type CK_MECHANISM_TYPE_PTR = *const CK_MECHANISM_TYPE;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct CK_MECHANISM_INFO {
    pub ulMinKeySize: CK_ULONG,
    pub ulMaxKeySize: CK_ULONG,
    pub flags: CK_FLAGS,
}

#[allow(non_camel_case_types)]
pub type CK_MECHANISM_INFO_PTR = *const CK_MECHANISM_INFO;

#[allow(non_camel_case_types)]
pub type C_Initialize = extern "C" fn(CK_C_INITIALIZE_ARGS_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_Finalize = extern "C" fn(CK_VOID_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetInfo = extern "C" fn(CK_INFO_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetFunctionList = extern "C" fn(CK_FUNCTION_LIST_PTR_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetSlotList = extern "C" fn(CK_BBOOL, CK_SLOT_ID_PTR, CK_ULONG_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetSlotInfo = extern "C" fn(CK_SLOT_ID, CK_SLOT_INFO_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetTokenInfo = extern "C" fn(CK_SLOT_ID, CK_TOKEN_INFO_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetMechanismList = extern "C" fn(CK_SLOT_ID, CK_MECHANISM_TYPE_PTR, CK_ULONG_PTR) -> CK_RV;
#[allow(non_camel_case_types)]
pub type C_GetMechanismInfo = extern "C" fn(CK_SLOT_ID, CK_MECHANISM_TYPE, CK_MECHANISM_INFO_PTR) -> CK_RV;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug,Clone)]
#[repr(C)]
pub struct CK_FUNCTION_LIST {
    pub version: CK_VERSION,
    pub C_Initialize: Option<C_Initialize>,
    pub C_Finalize: Option<C_Finalize>,
    pub C_GetInfo: Option<C_GetInfo>,
    pub C_GetFunctionList: Option<C_GetFunctionList>,
    pub C_GetSlotList: Option<C_GetSlotList>,
    pub C_GetSlotInfo: Option<C_GetSlotInfo>,
    pub C_GetTokenInfo: Option<C_GetTokenInfo>,
    pub C_GetMechanismList: Option<C_GetMechanismList>,
    pub C_GetMechanismInfo: Option<C_GetMechanismInfo>,
}

#[allow(non_camel_case_types)]
pub type CK_FUNCTION_LIST_PTR = *const CK_FUNCTION_LIST;
#[allow(non_camel_case_types)]
pub type CK_FUNCTION_LIST_PTR_PTR = *const CK_FUNCTION_LIST_PTR;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Module(&'static str),
    Pkcs11(CK_RV),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Module(ref err) => write!(f, "PKCS#11 Module error: {}", err),
            Error::Pkcs11(ref err) => write!(f, "PKCS#11 error: 0x{:x}", err),
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug)]
pub struct Ctx {
  lib: libloading::Library,
  _is_initialized: bool,
  C_Initialize: C_Initialize,
  C_Finalize: C_Finalize,
  C_GetInfo: C_GetInfo,
  C_GetFunctionList: C_GetFunctionList,
  C_GetSlotList: C_GetSlotList,
  C_GetSlotInfo: C_GetSlotInfo,
  C_GetTokenInfo: C_GetTokenInfo,
  C_GetMechanismList: C_GetMechanismList,
  C_GetMechanismInfo: C_GetMechanismInfo,
}

impl Ctx {
    pub fn new(filename: &'static str) -> Result<Ctx, Error> {
        unsafe {
            let lib = libloading::Library::new(filename)?;
            let mut list: CK_FUNCTION_LIST_PTR = std::mem::uninitialized();
            {
                let func: libloading::Symbol<unsafe extern "C" fn(CK_FUNCTION_LIST_PTR_PTR) -> CK_RV> = lib.get(b"C_GetFunctionList")?;
                match func(&mut list) {
                    0 => (),
                    err => return Err(Error::Pkcs11(err)),
                }
            }

            // TODO: The following part is a bit awkward: we want to ensure that every pointer was indeed set,
            // so instead of matching in the function calls, we do it now, and add them explicitly to the Ctx
            // There is an interesting nightly feature for '?' support on Option (std::option::NoneError).

            let c_initialize = match (*list).C_Initialize {
                Some(func) => func,
                None => return Err(Error::Module("C_Initialize function not found")),
            };

            let c_finalize = match (*list).C_Finalize {
                Some(func) => func,
                None => return Err(Error::Module("C_Finalize function not found")),
            };

            let c_getinfo = match (*list).C_GetInfo {
                Some(func) => func,
                None => return Err(Error::Module("C_GetInfo function not found")),
            };

            let c_getfunctionlist = match (*list).C_GetFunctionList {
                Some(func) => func,
                None => return Err(Error::Module("C_GetFunctionList function not found")),
            };

            let c_getslotlist = match (*list).C_GetSlotList {
                Some(func) => func,
                None => return Err(Error::Module("C_GetSlotList function not found")),
            };

            let c_getslotinfo = match (*list).C_GetSlotInfo {
                Some(func) => func,
                None => return Err(Error::Module("C_GetSlotInfo function not found")),
            };

            let c_gettokeninfo = match (*list).C_GetTokenInfo {
                Some(func) => func,
                None => return Err(Error::Module("C_GetTokenInfo function not found")),
            };

            let c_getmechanismlist = match (*list).C_GetMechanismList {
                Some(func) => func,
                None => return Err(Error::Module("C_GetMechanismList function not found")),
            };

            let c_getmechanisminfo = match (*list).C_GetMechanismInfo {
                Some(func) => func,
                None => return Err(Error::Module("C_GetMechanismInfo function not found")),
            };

            Ok(Ctx {
                lib: lib,
                _is_initialized: false,
                C_Initialize: c_initialize,
                C_Finalize: c_finalize,
                C_GetInfo: c_getinfo,
                C_GetFunctionList: c_getfunctionlist,
                C_GetSlotList: c_getslotlist,
                C_GetSlotInfo: c_getslotinfo,
                C_GetTokenInfo: c_gettokeninfo,
                C_GetMechanismList: c_getmechanismlist,
                C_GetMechanismInfo: c_getmechanisminfo,
            })
        }
    }

    pub fn new_and_initialize(filename: &'static str) -> Result<Ctx, Error> {
        let mut ctx = Ctx::new(filename)?;
        ctx.initialize(None)?;
        Ok(ctx)
    }

    pub fn is_initialized(&self) -> bool {
        self._is_initialized
    }

    pub fn initialize(&mut self, init_args: Option<CK_C_INITIALIZE_ARGS>) -> Result<(), Error> {
        if self._is_initialized {
            return Err(Error::Module("module already initialized"))
        }
        match (self.C_Initialize)(&init_args.unwrap_or(CK_C_INITIALIZE_ARGS::new())) {
            0 => {
                self._is_initialized = true;
                Ok(())
            },
            err => Err(Error::Pkcs11(err)),
        }
    }

    pub fn finalize(&mut self) -> Result<(), Error> {
        if !self._is_initialized {
            return Err(Error::Module("module not initialized"))
        }
        match (self.C_Finalize)(ptr::null()) {
            0 => {
                self._is_initialized = false;
                Ok(())
            },
            err => Err(Error::Pkcs11(err)),
        }
    }

    pub fn get_info(&self) -> Result<CK_INFO, Error> {
        if !self._is_initialized {
            return Err(Error::Module("module not initialized"))
        }
        let mut info = Box::new(CK_INFO::new());
        match (self.C_GetInfo)(&mut *info) {
            0 => {
                Ok(*info)
            },
            err => Err(Error::Pkcs11(err)),
        }
    }

    pub fn get_function_list(&self) -> Result<CK_FUNCTION_LIST, Error> {
        let list: CK_FUNCTION_LIST_PTR = unsafe { std::mem::uninitialized() };
        match (self.C_GetFunctionList)(&list) {
            0 => {
                unsafe { Ok((*list).clone()) }
            },
            err => Err(Error::Pkcs11(err)),
        }
    }
}

impl Drop for Ctx {
    fn drop(&mut self) {
        if self.is_initialized() {
            if let Err(err) = self.finalize() {
                println!("ERROR: {}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /// Tests need to be run with `RUST_TEST_THREADS=1` currently to pass.

    use super::*;

    const PKCS11_MODULE_FILENAME: &'static str = "/usr/local/lib/softhsm/libsofthsm2.so";

    #[test]
    fn ctx_new() {
        let res = Ctx::new(PKCS11_MODULE_FILENAME);
        assert!(res.is_ok(), "failed to create new context: {:?}", res);
    }

    #[test]
    fn ctx_initialize() {
        let mut ctx = Ctx::new(PKCS11_MODULE_FILENAME).unwrap();
        let res = ctx.initialize(None);
        assert!(res.is_ok(), "failed to initialize context: {:?}", res);
        assert!(ctx.is_initialized(), "internal state is not initialized");
    }

    #[test]
    fn ctx_new_and_initialize() {
        
        let res = Ctx::new_and_initialize(PKCS11_MODULE_FILENAME);
        assert!(res.is_ok(), "failed to create or initialize new context: {:?}", res);
    }

    #[test]
    fn ctx_finalize() {
        let mut ctx = Ctx::new_and_initialize(PKCS11_MODULE_FILENAME).unwrap();
        let res = ctx.finalize();
        assert!(res.is_ok(), "failed to finalize context: {:?}", res);
    }

    #[test]
    fn ctx_get_info() {
        let ctx = Ctx::new_and_initialize(PKCS11_MODULE_FILENAME).unwrap();
        let res = ctx.get_info();
        assert!(res.is_ok(), "failed to call C_GetInfo: {:?}", res);
        let info = res.unwrap();
        println!("{:?}", info);
    }

    #[test]
    fn ctx_get_function_list() {
        let ctx = Ctx::new_and_initialize(PKCS11_MODULE_FILENAME).unwrap();
        let res = ctx.get_function_list();
        assert!(res.is_ok(), "failed to call C_GetFunctionList: {:?}", res);
        let list = res.unwrap();
        println!("{:?}", list);
    }
}
