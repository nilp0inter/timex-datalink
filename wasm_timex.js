// wasm_timex.js - Bridge between WebAssembly and JavaScript

let wasmModule = null;

// Function to initialize the WebAssembly module
async function initWasm() {
    try {
        // Import the wasm-bindgen generated JS
        const wasm = await import('./timex_datalink_wasm.js');
        
        // Initialize the module
        await wasm.default();
        wasmModule = wasm;
        
        console.log('WebAssembly module loaded successfully');
        return true;
    } catch (error) {
        console.error('Failed to load WebAssembly module:', error);
        throw error;
    }
}

// Function to generate Protocol 3 packets using the WebAssembly module
function generateProtocol3Packets() {
    if (!wasmModule) {
        throw new Error('WebAssembly module not initialized');
    }
    
    try {
        console.log('Checking if demo packets work...');
        const demoPackets = wasmModule.generate_demo_packets();
        console.log('Demo packets generated:', demoPackets);
        
        console.log('Calling generate_protocol3_packets from WebAssembly...');
        
        // Call the exported function from our Rust WASM module
        // This now returns a Result that we need to handle
        const result = wasmModule.generate_protocol3_packets();
        
        console.log('Packets generated successfully');
        
        if (Array.isArray(result)) {
            console.log('Number of packets:', result.length);
        } else {
            console.log('Result is not an array, using demo packets instead');
            return demoPackets;
        }
        
        // The expected return format is an array of arrays of bytes
        return result;
    } catch (error) {
        console.error('Error generating packets:', error);
        // Use demo packets as fallback
        console.log('Using demo packets as fallback');
        return wasmModule.generate_demo_packets();
    }
}

// Export the functions for use in the main script
window.initWasm = initWasm;
window.generateProtocol3Packets = generateProtocol3Packets;