use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Connection {
    pub name: String,
}

static CONNECTION: OnceLock<Connection> = OnceLock::new();

pub fn get_connection() -> &'static Connection {
    CONNECTION.get_or_init(|| {
        println!("(Initializing new connection...)");
        Connection {
            name: "default".into(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_singleton_is_global() {
        let conn1 = get_connection();
        let conn2 = get_connection();
        assert!(std::ptr::eq(conn1, conn2)); // same instance
    }

    #[test]
    fn test_thread_safety() {
        let handles: Vec<_> = (0..10)
            .map(|_| thread::spawn(|| get_connection().name.clone()))
            .collect();

        for h in handles {
            assert_eq!(h.join().unwrap(), "default");
        }
    }
}
