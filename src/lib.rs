#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X11DisplayInfo {
    pub display_number: u32,
    pub socket_path: String,
}

impl X11DisplayInfo {
    pub fn default_display() -> Self {
        Self { display_number: 0, socket_path: "/tmp/.X11-unix/X0".into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_x11_display_info_aaa() {
        // Arrange & Act
        let disp = X11DisplayInfo::default_display();
        // Assert
        assert_eq!(disp.display_number, 0);
        assert_eq!(disp.socket_path, "/tmp/.X11-unix/X0");
    }
}
