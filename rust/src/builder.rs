#[derive(Debug, PartialEq)]
pub struct Computer {
    cpu: String,
    ram_gb: u32,
    gpu: Option<String>,
    storage_gb: Option<u32>,
}

pub struct ComputerBuilder {
    cpu: String,
    ram_gb: u32,
    gpu: Option<String>,
    storage_gb: Option<u32>,
}

impl ComputerBuilder {
    fn new(cpu: String, ram_gb: u32) -> Self {
        ComputerBuilder {
            cpu,
            ram_gb,
            gpu: None,
            storage_gb: None,
        }
    }

    fn add_gpu(mut self, gpu: String) -> Self {
        self.gpu = Some(gpu);
        self
    }

    fn add_storage(mut self, storage_gb: u32) -> Self {
        self.storage_gb = Some(storage_gb);
        self
    }

    fn build(self) -> Computer {
        Computer {
            cpu: self.cpu,
            ram_gb: self.ram_gb,
            gpu: self.gpu,
            storage_gb: self.storage_gb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_computer_build() {
        let computer = ComputerBuilder::new("Intel i5".to_string(), 16).build();

        assert_eq!(
            computer,
            Computer {
                cpu: "Intel i5".to_string(),
                ram_gb: 16,
                gpu: None,
                storage_gb: None,
            }
        );
    }

    #[test]
    fn test_full_computer_build() {
        let computer = ComputerBuilder::new("AMD Ryzen 9".to_string(), 64)
            .add_gpu("NVIDIA RTX 4090".to_string())
            .add_storage(2048)
            .build();

        assert_eq!(
            computer,
            Computer {
                cpu: "AMD Ryzen 9".to_string(),
                ram_gb: 64,
                gpu: Some("NVIDIA RTX 4090".to_string()),
                storage_gb: Some(2048),
            }
        );
    }

    #[test]
    fn test_computer_with_gpu_only() {
        let computer = ComputerBuilder::new("Intel i9".to_string(), 32)
            .add_gpu("AMD RX 7900 XTX".to_string())
            .build();

        assert_eq!(
            computer,
            Computer {
                cpu: "Intel i9".to_string(),
                ram_gb: 32,
                gpu: Some("AMD RX 7900 XTX".to_string()),
                storage_gb: None,
            }
        );
    }
}
