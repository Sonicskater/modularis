use std::slice::from_raw_parts_mut;
use wasmtime::Store;

pub fn write_bytes_to_wasm_memory<T : WasmMemory>(bytes: &[u8], memory: &T, ptr: usize, len: usize){
    let mem_array: &mut [u8];
    unsafe {
        mem_array = memory.data_unchecked_mut();
        for i in 0..len {
            // iterate over the serialized struct, copying it to the memory of the target module,
            // using the ptr provided by caller
            mem_array[ptr + i as usize] = bytes[i as usize];
        }
    }
}

#[cfg(feature = "bincode-ffi")]
pub fn write_bincode_to_wasm_memory<T : serde::Serialize, M : WasmMemory>(data: T, memory: &M, ptr: usize, len: usize){
    let serialized_array = bincode::serialize(&data).expect("Failed to serialize type");
    write_bytes_to_wasm_memory(&*serialized_array, memory, ptr, len)
}

#[cfg(feature = "bytemuck-ffi")]
pub fn write_bytemuck_to_wasm_memory<T : bytemuck::Pod, M : WasmMemory>(data: T, memory: &M, ptr: usize, len: usize){
    let bytes = bytemuck::bytes_of(&data);
    //println!("Writing {} bytes using Bytemuck",bytes.len());
    write_bytes_to_wasm_memory(bytes, memory, ptr, len)
}

pub trait WasmMemory {
    unsafe fn data_unchecked_mut(&self) -> &mut [u8];
}

#[cfg(all(feature = "wasmer"))]
impl WasmMemory for wasmer::Memory {
    unsafe fn data_unchecked_mut(&self) -> &mut [u8] {
        self::data_unchecked_mut()
    }
}

#[cfg(all(feature = "wasmtime"))]
pub struct WasmtimeMemory<'a,T>{
    store : &'a wasmtime::Store<T>,
    memory : &'a wasmtime::Memory
}

#[cfg(all(feature = "wasmtime"))]
impl WasmtimeMemory<_> {
    pub fn new<'a,T>(
        store: &'a wasmtime::Store<T>,
        memory: &'a wasmtime::Memory
    ) -> WasmtimeMemory<'a,T>{

        WasmtimeMemory{
            store,
            memory
        }
    }
}


#[cfg(all(feature = "wasmtime"))]
impl WasmMemory for WasmtimeMemory<_> {
    unsafe fn data_unchecked_mut(&self) -> &mut [u8] {
        unsafe {
            let ptr = self.memory.data_ptr(self.store);
            let len = self.memory.data_size(self.store);
            from_raw_parts_mut(ptr,len)
        }
    }
}

