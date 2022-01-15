

#[cfg(all(feature = "wasmer-backend",feature = "cstruct"))]
pub fn write_bytes_to_wasm_memory(bytes: &[u8], memory: &Memory, ptr: usize, len: usize){
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