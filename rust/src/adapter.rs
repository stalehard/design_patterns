pub trait Show {
    fn show(&self) -> String;
}

pub struct StringTarget {
    value: String,
}

impl Show for StringTarget {
    fn show(&self) -> String {
        format!("StringTarget [{}]", self.value)
    }
}

pub struct DigitalTarget<'t> {
    value: &'t [i32],
}

pub struct Adapter<'t> {
    obj: DigitalTarget<'t>,
}

impl<'t> Adapter<'t> {
    pub fn new(obj: DigitalTarget<'t>) -> Self {
        Adapter { obj }
    }
}

impl<'t> Show for Adapter<'t> {
    fn show(&self) -> String {
        format!(
            "DigitalTarget [{}]",
            self.obj
                .value
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

fn call<T: Show>(obj: &T) {
    obj.show();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_target() {
        let target = StringTarget {
            value: "Hello".to_string(),
        };
        assert_eq!(target.show(), "StringTarget [Hello]");
    }

    #[test]
    fn test_adapter() {
        let target = DigitalTarget { value: &[1, 2, 3] };
        let adapter = Adapter::new(target);
        assert_eq!(adapter.show(), "DigitalTarget [1, 2, 3]");
    }

    #[test]
    fn test_call_adapter_show() {
        let target = DigitalTarget { value: &[1, 2, 3] };
        let adapter = Adapter::new(target);
        call(&adapter);
    }
}
