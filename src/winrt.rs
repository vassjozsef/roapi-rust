use ari::os::win::{Library, Symbol};
use lazy_static::lazy_static;
use windows::core::HRESULT;

static WINRT_LIBRARY_NAME: &'static str = "api-ms-win-core-winrt-l1-1-0.dll";

static WINRT_ROINITIALIZE_FUNCTION_NAME: &'static [u8; 13usize] = b"RoInitialize\0";

pub type RoInitializeFn = unsafe extern "system" fn(initType: i32) -> i32;

fn invoke<TFn>(function: TFn) -> Result<(), HRESULT>
where
    TFn: FnOnce() -> i32,
{
    match (function)() {
        0 => Ok(()),
        e => Err(HRESULT(e)),
    }
}

lazy_static! {
    static ref FUN_RO_INITIALIZE: Result<Symbol<RoInitializeFn>, std::io::Error> = {
        let library = Library::open(WINRT_LIBRARY_NAME)?;
        let ro_initialize_fn: Symbol<RoInitializeFn> =
            unsafe { library.find(WINRT_ROINITIALIZE_FUNCTION_NAME) }?;
        Ok(ro_initialize_fn)
    };
}

pub fn ro_initialize() {
    println!("ro_initialize");
    let fun = FUN_RO_INITIALIZE.as_ref().ok().unwrap();
    dbg!(fun);
    let result = unsafe { invoke(|| (fun)(1)) };
    dbg!(&result);
    result.ok();
}
