fn main() {
    println!("cargo:rerun-if-changed=src/app/dtos");
    println!("cargo:rerun-if-changed=templates/");
    
    // The TypeScript types will be generated at runtime when the app starts
    // This is just to ensure the build system tracks changes to the DTOs
} 