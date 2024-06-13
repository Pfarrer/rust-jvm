use crate::prelude::VmFrame;

impl std::fmt::Display for VmFrame {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Method name
        fmt.write_str("Frame of ")?;
        fmt.write_str(&self.class_path)?;
        fmt.write_str(".")?;
        fmt.write_str(&self.method_name)?;
        fmt.write_str(&self.method_signature)?;
        fmt.write_str("\nStack:\n")?;

        /*        for primitive in self.stack.iter() {
                    let desc = format!("  - {}\n", primitive);
                    fmt.write_str(&desc)?;
                }
        */
        Ok(())
    }
}
